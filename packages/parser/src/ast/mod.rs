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
    AssignDef(AssignDef),
    HandlerDef(HandlerDef),
    HandlerAssign(HandlerAssign),
    TraitDef(TraitDef),
    InstDef(InstDef),
    LineComment(LineComment),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerDef {
    pub eta_envs: EtaEnvs,
    pub ident: HandlerIdent,
    pub expr: HandlerTypeDefExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerTypeDefExpr {
    pub constraints: Vec<Constraint>,
    pub handler_expr: HandlerTypeExpr,
    pub expr: TypeAbstructionExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstDef {
    pub ident: InstIdent,
    pub expr: TypeExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtaEnvs(pub Vec<EtaEnv>);

#[derive(Debug, Serialize, Deserialize)]
pub struct EtaEnv {
    pub ident: HandlerIdent,
    pub expr: HandlerTypeExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Apply(Apply),
    ApplyInst(ApplyInst),
    ApplyEff(ApplyEff),
    Ident(Ident),
    InstIdent(InstIdent),
    HandlerIdent(HandlerIdent),
    Literal(Literal),
    Abstruction(Abstruction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Abstruction {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerTypeExpr {
    pub resume: TypeAbstructionExpr,
    pub ret: TypeAbstructionExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitDef {
    pub constraints: Vec<Constraint>,
    pub ident: TypeIdent,
    pub args: Vec<TypeIdent>,
    pub where_clause: Module,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraint {
    pub ident: TypeIdent,
    pub args: Vec<TypeIdent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apply {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyInst {
    pub ident: InstIdent,
    pub expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyEff {
    pub ident: HandlerIdent,
    pub expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeExpr {
    pub constraints: Vec<Constraint>,
    pub expr: TypeAbstructionExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeAbstructionExpr {
    Arrow(Box<TypeAbstructionExpr>, Box<TypeAbstructionExpr>),
    Literal(TypeLiteral),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeLiteral {
    Top,
    Array(Box<TypeAbstructionExpr>),
    Context(TypeIdent, Box<TypeAbstructionExpr>),
    Tuple(usize, Vec<TypeAbstructionExpr>),
    Ident(TypeIdent),
    Bottom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    pub ident: Ident,
    pub args: AssignArgs,
    pub expr: Expr,
    pub where_clause: Module,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerAssign {
    pub ident: HandlerIdent,
    pub args: AssignArgs,
    pub expr: Expr,
    pub where_clause: Module,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignArgs {
    pub patterns: Vec<PatternExpr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatternExpr {
    Or(Box<PatternExpr>, Box<PatternExpr>),
    Literal(Literal),
    Bind(Ident),
    ListHead(),
    Tuple(usize, Vec<PatternExpr>),
    Any,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignDef {
    pub eta_envs: EtaEnvs,
    pub ident: Ident,
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
pub struct HandlerIdent(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct InstIdent(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Char(char),
    Text(String),
    Int(isize),
    Unit,
    Array(Vec<Expr>),
    Tuple(usize, Vec<Expr>),
}
