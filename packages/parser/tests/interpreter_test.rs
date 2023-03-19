extern crate parser;

use insta::{assert_debug_snapshot, glob};
use std::fs;

#[test]
fn run_test() {
    glob!("../fixtures/pds", "*.pds", |path| {
        let input = fs::read_to_string(path).unwrap();
        let value = parser::prose_down_script_run(&input).unwrap();

        assert_debug_snapshot!(value);
    });
}
