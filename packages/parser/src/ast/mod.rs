use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pds(pub Vec<Assign>);

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Assign(Ident, Vec<Ident>, Box<Expr>),
    Literal(Literal),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign(pub Ident, pub Vec<Ident>, pub Box<Expr>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Text(String),
    Int(isize),
}

#[derive(Serialize, Deserialize)]
pub enum Block {
    PdsScript(String),
    Paragraph(Vec<Inline>),
    EmptyLine,
    ThemanticBreak,
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

#[derive(Serialize, Deserialize)]
pub enum Inline {
    Text(String),
    Number(String),
    Expr(String),
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
