use handlebars::JsonValue;
use std::{collections::BTreeMap, path::PathBuf};

pub trait Node {
    fn name(&self) -> &str;
    fn data(&self) -> &BTreeMap<String, JsonValue>;
    fn node_type(&self) -> NodeType;
    fn location(&self) -> Option<Location> {
        None
    }
}

pub trait Parent: Node {
    fn iter(&self) -> impl Iterator<Item = &dyn Node>;
}

pub trait Literal: Node {
    fn value(&self) -> &JsonValue;
}

pub enum NodeType {
    Terminal,
    Parent,
    Literal,
}
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}
