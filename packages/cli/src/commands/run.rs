use crate::args::Run;
use anyhow::Result;
use gokuraku_config::GokurakuConfigInstance;
use parser::prose_down_script_run;
use std::fs;

pub(crate) fn run(_conf: &GokurakuConfigInstance, options: &Run) -> Result<()> {
    prose_down_script_run(&fs::read_to_string(&options.file)?)?;

    Ok(())
}
