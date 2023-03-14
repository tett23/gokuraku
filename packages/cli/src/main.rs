mod args;
mod commands;

use anyhow::{Context, Result};
use args::Args;
use args::Commands;
use gokuraku_config::{GokurakuConfig, GokurakuConfigInstance};
use std::fs;

fn main() {
    let args = args::parse();
    let config = match fetch_config(&args) {
        Ok(config) => config,
        Err(e) => {
            panic!("{e}")
        }
    };

    match args.command {
        Commands::Build(_) => commands::build(&config),
        Commands::Run(options) => commands::run(&config, &options),
    }
    .unwrap();
}

fn fetch_config(args: &Args) -> Result<GokurakuConfigInstance> {
    let conf = match &args.config {
        Some(path) => fs::read_to_string(path)
            .context("")
            .map(|toml| GokurakuConfig::from_toml_str(&toml))?,
        None => Ok(GokurakuConfig::default()),
    }?;

    GokurakuConfigInstance::try_from((conf, args.into()))
}

impl From<&Args> for gokuraku_config::CLIArgs {
    fn from(value: &Args) -> Self {
        Self {
            env: value.env.clone(),
            output: value.output.clone(),
            formats: value.format.clone(),
            input: value.input.clone(),
        }
    }
}
