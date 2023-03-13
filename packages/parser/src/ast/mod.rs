use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pds(pub Vec<TopLevel>);

#[derive(Debug, Serialize, Deserialize)]
pub enum TopLevel {
    Assign(Assign),
    AssignAnnotation(AssignAnnotation),
    LineComment(LineComment),
    Environment(),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Apply(Ident, Box<Expr>),
    Literal(Literal),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeExpr {
    Assign(Ident, Vec<Ident>, Box<Expr>),
    Literal(Literal),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    pub ident: Ident,
    pub args: AssignArgs,
    pub expr: Expr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignArgs {
    pub patterns: Vec<PatternExpr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatternExpr {
    Bind(Ident),
    TypeBind(TypeIdent),
    Literal(Literal),
    // SplitList
    Any,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignAnnotation {
    pub ident: Ident,
    pub conditions: Vec<ParameterCondition>,
    pub expr: TypeExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineComment(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterCondition {
    type_class: TypeClass,
    ident: Ident,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeClass {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeIdent(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Char(char),
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
