extern crate bytecode;

use anyhow::Result;
use std::fs;

#[test]
#[ignore]
fn test_parse() -> Result<()> {
    matches!(
        bytecode::parse(&fs::read_to_string("fixtures/1.pirt")?),
        Ok(_)
    );

    Ok(())
}
