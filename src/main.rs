use anyhow::Result;
use clap::Parser;
use crates::print_krates;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    args::Command,
    crates::{get_all_krates, upgrade_krates},
};

mod args;
mod crates;

fn main() -> Result<()> {
    let Command::Installed(args) = Command::parse();
    let filter = if args.verbose {
        "cargo_installed=debug,ureq=info"
    } else {
        "cargo_installed=info,ureq=error"
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| filter.into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!(?args);

    let krates = get_all_krates()?;
    print_krates(&krates, args.outdated);
    if args.upgrade {
        upgrade_krates(&krates, &args.ignore, args.ignore_local)?;
    }

    Ok(())
}
