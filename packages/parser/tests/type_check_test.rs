extern crate parser;

use insta::{assert_debug_snapshot, glob};
use std::fs;

#[test]
fn type_check_test() {
    glob!("../fixtures/pds", "*.pds", |path| {
        dbg!(&path);
        let input = fs::read_to_string(path).unwrap();
        let ast = parser::prose_down_script_parse(&input).unwrap();
        let ir1 = parser::ir::ir1::transform1(&ast);
        let ir2 = parser::ir::ir2::transform2(ir1, ast);

        assert_debug_snapshot!(ir2);
    });
}
