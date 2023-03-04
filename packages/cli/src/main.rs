use std::{env, fs};

fn main() {
    let input = &env::args().collect::<Vec<String>>()[1];

    let content = fs::read_to_string(input).unwrap();
    prose_down::parse(&content);
}
