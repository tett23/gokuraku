extern crate parser;

use insta::{assert_debug_snapshot, glob};
use std::fs;

#[test]
fn pds_test() {
    glob!("../fixtures/pds", "*.pds", |path| {
        let input = fs::read_to_string(path).unwrap();
        let ast = parser::prose_down_script_parse(&input).unwrap();

        assert_debug_snapshot!(ast);
    });
}
