use crate::args::BuildPds;
use anyhow::Result;
use gokuraku_config::GokurakuConfigInstance;
use parser::prose_down_parse;
use std::fs;

pub(crate) fn build_pds(_conf: &GokurakuConfigInstance, options: &BuildPds) -> Result<()> {
    let result = prose_down_parse(&fs::read_to_string(&options.file)?)?;
    dbg!(result);

    Ok(())
}
