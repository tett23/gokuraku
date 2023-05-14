use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document(pub Vec<Block>);

impl Document {
    pub fn iter(&self) -> impl Iterator<Item = &Block> {
        self.0.iter()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Block {
    PdsScript(String),
    Paragraph(Vec<Inline>),
    EmptyLine,
    ThemanticBreak,
}

impl Block {
    pub fn iter(&self) -> impl Iterator<Item = &Inline> {
        match self {
            Self::PdsScript(_) => std::iter::empty(),
            Self::Paragraph(value) => std::iter::empty(),
            Self::ThemanticBreak => std::iter::empty(),
            Self::EmptyLine => std::iter::empty(),
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PdsScript(value) => write!(f, "PdsScript({value})"),
            Self::Paragraph(value) => {
                write!(f, "Paragraph(\n\t")?;
                value.iter().try_for_each(|item| item.fmt(f))?;
                write!(f, "\n)")
            }
            Self::ThemanticBreak => write!(f, "ThemanticBreak"),
            Self::EmptyLine => write!(f, "EmptyLine"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Inline {
    Text(String),
    Number(String),
    Expr(String),
}

impl Inline {
    pub fn iter(&self) -> impl Iterator<Item = &Inline> {
        match self {
            Self::Text(_) => std::iter::empty(),
            Self::Number(_) => std::iter::empty(),
            Self::Expr(_) => std::iter::empty(),
        }
    }
}

impl Debug for Inline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(value) if matches!(value.as_str(), "\r\n" | "\n" | "\r") => {
                write!(f, "")
            }
            Self::Text(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "Number(##{value}##)"),
            Self::Expr(value) => write!(f, "Expr({{{value}}})"),
        }
    }
}
