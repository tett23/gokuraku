extern crate parser;

use insta::{assert_debug_snapshot, glob};
use parser::{ir::Module, vm::Vm};
use std::fs;

#[test]
fn run_test() {
    glob!("../fixtures/pds", "*.pds", |path| {
        let input = fs::read_to_string(path).unwrap();
        let ast = parser::prose_down_script_parse(&input).unwrap();
        let mut vm = Vm::default();
        let value = vm.run(ast.into());

        assert_debug_snapshot!(value);
    });
}
