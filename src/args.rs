//! Handle `cargo installed` arguments
use clap::Parser;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
pub enum Command {
    /// Easy manage programs installed by `cargo install`.
    #[command(name = "installed")]
    Installed(Args),
}

#[derive(Debug, Default, Parser)]
pub struct Args {
    /// upgrade all outdated
    #[arg(short, long)]
    pub upgrade: bool,

    /// show outdated
    #[arg(short, long)]
    pub outdated: bool,

    /// ignore installed from local space, enabled by default
    #[arg(short = 'l', long, default_value_t = true)]
    pub ignore_local: bool,

    /// ignore from upgrade
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// show details
    #[arg(short, long)]
    pub verbose: bool,
}
