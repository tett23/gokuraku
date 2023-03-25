extern crate parser;

use insta::{assert_debug_snapshot, glob};
use std::fs;

#[test]
fn type_check_test() {
    glob!("../fixtures/pds", "*.pds", |path| {
        let input = fs::read_to_string(path).unwrap();
        let ast = parser::prose_down_script_parse(&input).unwrap();
        let ir = parser::ir::ir1::SymbolTable::try_from(ast);

        assert_debug_snapshot!(ir);
    });
}
