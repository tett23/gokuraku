use handlebars::JsonValue;
use std::{collections::BTreeMap, fmt::Debug, path::PathBuf};

pub trait Node {
    fn name(&self) -> &str;
    fn data(&self) -> &BTreeMap<String, JsonValue>;
    fn node_type(&self) -> NodeType;
    fn location(&self) -> Option<Location> {
        // TODO
        None
    }
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>>;
    fn value(&self) -> Option<&JsonValue> {
        None
    }
}

impl Debug for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.node_type() {
            NodeType::Terminal => write!(f, "Terminal({name})", name = self.name()),
            NodeType::Parent => {
                write!(f, "Parent({name})", name = self.name())?;
                self.iter().try_for_each(|item| item.fmt(f))?;
                Ok(())
            }
            NodeType::Literal => write!(f, "Literal({name})", name = self.name()),
        }
    }
}

pub trait Parent: Node {
    // fn iter(&self) -> impl Iterator<Item = &dyn Node>;
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
