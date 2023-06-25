use std::{cmp::Ordering, fmt::Display};

use anyhow::{bail, Result};
use semver::Version;
use tracing::debug;
use xshell::{cmd, Shell};

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KrateStatus {
    #[default]
    Unknown,
    Outdated,
    UpToDate,
    Ignored,
}

impl KrateStatus {
    #[inline]
    pub fn symbol(&self) -> &'static str {
        match self {
            KrateStatus::Unknown => "?",
            KrateStatus::Outdated => "✗",
            KrateStatus::UpToDate => "✓",
            KrateStatus::Ignored => ".",
        }
    }
}

impl Display for KrateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KrateStatus::Unknown => write!(f, "Unknow"),
            KrateStatus::Outdated => write!(f, "Outdated"),
            KrateStatus::UpToDate => write!(f, "UpToDate"),
            KrateStatus::Ignored => write!(f, "Ignored"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InstalledKrate {
    name: String,
    version: String,
    from: Option<String>,
    commands: Vec<String>,
}

impl InstalledKrate {
    pub fn get_latest_version(&self) -> Result<String> {
        let versions = get_crate_versions(&self.name)?;
        let krate_json = json::parse(&versions)?;
        let first = &krate_json["versions"][0];

        Ok(first["num"].to_string())
    }

    #[inline]
    pub fn is_local(&self) -> bool {
        if let Some(f) = &self.from {
            return f.starts_with('/');
        }
        false
    }
}

fn get_crate_versions(krate: &str) -> Result<String> {
    let url = format!("https://crates.io/api/v1/crates/{krate}/versions");
    debug!("ureq requesting: {}", &url);
    let body = ureq::get(&url)
        .set("User-Agent", USER_AGENT)
        .call()?
        .into_string()?;

    Ok(body)
}

pub fn get_installed() -> Result<Vec<InstalledKrate>> {
    let sh = Shell::new()?;
    debug!("running: cargo install --list");
    let output = cmd!(sh, "cargo install --list").read()?;
    debug!("cargo install --list output:\n {}", &output);
    let mut result = vec![];
    let mut krate_count = 0;
    let mut previous = InstalledKrate::default();
    for line in output.lines() {
        if !line.starts_with(' ') {
            krate_count += 1;
            if krate_count > 1 {
                result.push(previous.clone());
            }
            let splited: Vec<&str> = line.trim().trim_matches(':').split(' ').collect();
            previous.name = splited[0].to_string();
            previous.version = splited[1].trim_start_matches('v').to_string();
            previous.from = if splited.len() >= 3 {
                let parn: &[_] = &['(', ')'];
                Some(splited[2].trim_matches(parn).to_string())
            } else {
                None
            };
            previous.commands = vec![];
        } else {
            previous.commands.push(line.trim().to_string());
        }
    }
    result.push(previous);

    Ok(result)
}

#[derive(Debug, Default, Clone)]
pub struct Krate {
    pub installed: InstalledKrate,
    pub latest: String,
    pub status: KrateStatus,
}

impl Krate {
    #[inline]
    pub fn name(&self) -> &str {
        &self.installed.name
    }

    #[inline]
    pub fn version(&self) -> &str {
        &self.installed.version
    }

    #[inline]
    pub fn is_local(&self) -> bool {
        self.installed.is_local()
    }

    pub fn upgrade(&self) -> Result<()> {
        debug!("upgrading: {}", self.name());
        if KrateStatus::UpToDate == self.status {
            bail!("Already up to date!")
        }
        let sh = Shell::new()?;
        let name = &self.installed.name;
        cmd!(sh, "cargo install --force {name}").run()?;

        Ok(())
    }
}

pub fn get_all_krates(ignored: &[String], ignore_local: bool) -> Result<Vec<Krate>> {
    let mut result = vec![];
    let installed = get_installed()?;

    for i in &installed {
        let (latest, status) = if ignored.contains(&i.name) || (ignore_local && i.is_local()) {
            let s = KrateStatus::Ignored;
            (s.to_string().to_ascii_lowercase(), s)
        } else if let Ok(v) = i.get_latest_version() {
            let latest = Version::parse(&v).unwrap();
            let current = Version::parse(&i.version).unwrap();
            match latest.cmp(&current) {
                Ordering::Equal => (v, KrateStatus::UpToDate),
                Ordering::Less => (v, KrateStatus::UpToDate),
                Ordering::Greater => (v, KrateStatus::Outdated),
            }
        } else {
            let s = KrateStatus::Unknown;
            (s.to_string().to_ascii_lowercase(), s)
        };
        let krate = Krate {
            installed: i.clone(),
            latest,
            status,
        };
        result.push(krate);
    }

    Ok(result)
}

pub fn print_krates(krates: &[Krate], outdated_only: bool) {
    //compute max length of each segment
    let mut name = "name".len();
    let mut installed = "installed".len();
    let mut latest = "latest".len();

    for krate in krates {
        name = name.max(krate.installed.name.len());
        installed = installed.max(krate.installed.version.len());
        latest = latest.max(krate.latest.len());
    }

    //add padding to text
    name += 3;
    installed += 3;
    latest += 3;

    // print header
    if outdated_only {
        println!(
            "{:name$}{:installed$}{:latest$}",
            "Name", "Installed", "Latest"
        );
    } else {
        println!(
            "{:name$}{:installed$}{:latest$}{:6}",
            "Name", "Installed", "Latest", "Status"
        );
    }

    // print crates data
    for k in krates {
        if outdated_only {
            if k.status == KrateStatus::Outdated {
                println!(
                    "{:name$}{:installed$}{:latest$}",
                    k.installed.name, k.installed.version, k.latest
                );
            }
        } else {
            println!(
                "{:name$}{:installed$}{:latest$}{:^7}",
                k.installed.name,
                k.installed.version,
                k.latest,
                k.status.symbol()
            );
        }
    }
}

pub fn upgrade_krates(krates: &[Krate], ignored: &[String], ignore_local: bool) -> Result<()> {
    for k in krates {
        if ignored.contains(&k.name().to_owned()) {
            debug!("ignoring: {}", k.name());
            continue;
        }

        if k.status != KrateStatus::Outdated {
            debug!("ignoring not outdated: {}", k.name());
            continue;
        }

        if ignore_local && k.is_local() {
            debug!("ignoring installed from local: {}", k.name());
            continue;
        }

        println!(
            "Upgrading {} from {} to {} ...",
            k.name(),
            k.version(),
            k.latest
        );
        k.upgrade()?;
    }

    Ok(())
}
