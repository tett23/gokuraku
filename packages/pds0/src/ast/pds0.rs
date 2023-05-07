use super::{Literal, Node, NodeType, Parent};
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
}

#[derive(Debug, Clone)]
pub(crate) struct Pds0Parent {
    pub name: String,
    pub data: BTreeMap<String, JsonValue>,
    pub children: Vec<Pds0Ast>,
}

impl Pds0Parent {
    fn iter(&self) -> NodeIterator<Pds0Ast> {
        NodeIterator {
            index: 0,
            nodes: self.children.as_slice(),
        }
    }
}

pub(crate) struct NodeIterator<'a, T> {
    pub(crate) index: usize,
    pub(crate) nodes: &'a [T],
}

impl<'a, T: Node> Iterator for NodeIterator<'a, T> {
    type Item = &'a dyn Node;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.nodes.get(self.index);
        self.index += 1;

        ret.map(|item| item as Self::Item)
    }
}

impl Node for Pds0Parent {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> &BTreeMap<String, JsonValue> {
        &self.data()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Parent
    }
}

impl Parent for Pds0Parent {
    fn iter(&self) -> impl Iterator<Item = &dyn Node> {
        NodeIterator::<Pds0Ast> {
            index: 0,
            nodes: &self.children,
        }
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
        &self.data()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Literal
    }
}

impl Literal for Pds0Literal {
    fn value(&self) -> &JsonValue {
        &self.value
    }
}
