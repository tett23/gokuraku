extern crate parser;

use insta::{assert_debug_snapshot, glob};
use std::fs;

#[test]
fn prose_down_test() {
    glob!("../../../fixtures/prose-down/*.pd", |path| {
        let input = fs::read_to_string(path).unwrap();
        let ast = parser::prose_down_parse(&input).unwrap();

        assert_debug_snapshot!(ast);
    });
}
