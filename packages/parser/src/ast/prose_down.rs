use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document(pub Vec<Block>);

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
