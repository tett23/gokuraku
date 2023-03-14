use super::parse;
use crate::vm::Vm;
use anyhow::Result;

pub fn run(script: &str) -> Result<()> {
    let ast = parse(&script)?;
    let mut vm = Vm::default();

    vm.run(ast.into());

    Ok(())
}
