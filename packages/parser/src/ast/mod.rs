mod prose_down;

pub use self::prose_down::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    Assign(Assign),
    AssignAnnotation(AssignAnnotation),
    HandlerAssign(),
    LineComment(LineComment),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Apply(Apply),
    ApplyEmbedded(ApplyEmbedded),
    Ident(Ident),
    Literal(Literal),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apply {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyEmbedded {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeExpr {
    TypeAbstruction(TypeIdent, Box<TypeExpr>),
    Literal(TypeLiteral),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeLiteral {
    pub type_class: TypeClass,
    pub ident: Ident,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    pub ident: Ident,
    pub args: AssignArgs,
    pub expr: Expr,
    pub where_clause: Option<Box<Module>>,
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
    SplitList(),
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
    Unit,
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Abstruction(Ident, Expr);

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAbstruction(TypeIdent, TypeExpr);
