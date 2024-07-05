extern crate bytecode;

use anyhow::Result;
use std::fs;

#[test]
fn test_make_wat() -> Result<()> {
    let ir = bytecode::parse(&fs::read_to_string("fixtures/1.pirt")?)?;
    let wat = bytecode::make_wat(ir);
    wat.pretty_print();

    Ok(())
}
