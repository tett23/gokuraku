use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "subcommand", author, about, version)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
    #[clap(short, long)]
    pub(crate) config: Option<String>,
    #[clap(short, long, env, default_value = "development")]
    pub(crate) env: String,
    #[clap(short, long)]
    pub(crate) output: Option<PathBuf>,
    #[clap(short, long)]
    pub(crate) format: Option<Vec<String>>,
    #[clap(short, long)]
    pub(crate) input: Option<String>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Build(Build),
}

#[derive(Debug, clap::Args)]
pub(crate) struct Build {}

pub(crate) fn parse() -> Args {
    Args::parse()
}
