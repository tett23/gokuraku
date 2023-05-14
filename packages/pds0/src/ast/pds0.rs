use super::{Node, NodeType};
use handlebars::JsonValue;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(crate) enum Pds0Ast {
    Node(Pds0Node),
    Parent(Pds0Parent),
    Literal(Pds0Literal),
}

impl Node for Pds0Ast {
    fn name(&self) -> &str {
        match self {
            Pds0Ast::Node(node) => &node.name,
            Pds0Ast::Parent(parent) => &parent.name,
            Pds0Ast::Literal(literal) => &literal.name,
        }
    }
    fn data(&self) -> &BTreeMap<String, JsonValue> {
        match self {
            Pds0Ast::Node(node) => &node.data,
            Pds0Ast::Parent(parent) => &parent.data,
            Pds0Ast::Literal(literal) => &literal.data,
        }
    }

    fn node_type(&self) -> NodeType {
        match self {
            Pds0Ast::Node(node) => NodeType::Terminal,
            Pds0Ast::Parent(parent) => NodeType::Parent,
            Pds0Ast::Literal(literal) => NodeType::Literal,
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>> {
        match self {
            Pds0Ast::Node(node) => node.iter(),
            Pds0Ast::Parent(parent) => parent.iter(),
            Pds0Ast::Literal(literal) => literal.iter(),
        }
    }

    fn value(&self) -> Option<&JsonValue> {
        match self {
            Pds0Ast::Node(node) => node.value(),
            Pds0Ast::Parent(parent) => parent.value(),
            Pds0Ast::Literal(literal) => literal.value(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Pds0Node {
    pub name: String,
    pub data: BTreeMap<String, JsonValue>,
}

impl Node for Pds0Node {
    fn data(&self) -> &BTreeMap<String, JsonValue> {
        &self.data
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn node_type(&self) -> NodeType {
        NodeType::Terminal
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>> {
        Box::new(std::iter::empty())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Pds0Parent {
    pub name: String,
    pub data: BTreeMap<String, JsonValue>,
    pub children: Vec<Pds0Ast>,
}

impl Pds0Parent {}

impl IntoIterator for Pds0Parent {
    type Item = Pds0Ast;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl Node for Pds0Parent {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> &BTreeMap<String, JsonValue> {
        &self.data
    }

    fn node_type(&self) -> NodeType {
        NodeType::Parent
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>> {
        Box::new(
            self.children
                .clone()
                .into_iter()
                .map(|item| -> Box<dyn Node> { Box::new(item) }),
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Pds0Literal {
    pub name: String,
    pub data: BTreeMap<String, JsonValue>,
    pub value: JsonValue,
}

impl Node for Pds0Literal {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> &BTreeMap<String, JsonValue> {
        &self.data
    }

    fn node_type(&self) -> NodeType {
        NodeType::Literal
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>> {
        Box::new(std::iter::empty())
    }

    fn value(&self) -> Option<&JsonValue> {
        Some(&self.value)
    }
}
