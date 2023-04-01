mod prose_down;

pub use self::prose_down::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub statements: Vec<Statement>,
}

impl Module {
    pub fn iter(&self) -> impl Iterator<Item = &Statement> {
        self.statements.iter()
    }
}

impl IntoIterator for Module {
    type Item = Statement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.statements.into_iter()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    Assign(Assign),
    AssignDef(AssignDef),
    HandlerDef(HandlerDef),
    HandlerAssign(HandlerAssign),
    TraitDef(TraitDef),
    ImplTrait(ImplTrait),
    InstDef(InstDef),
    DataAssign(DataAssign),
    LineComment(LineComment),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerDef {
    pub ident: HandlerIdent,
    pub expr: HandlerTypeDefExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerTypeDefExpr {
    pub trait_constraints: Vec<TraitConstraint>,
    pub eta_envs: EtaEnvs,
    pub expr: TypeAbstructionExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstDef {
    pub ident: InstIdent,
    pub expr: TypeExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtaEnvs(pub Vec<EtaEnv>);

impl EtaEnvs {
    pub fn iter(&self) -> impl Iterator<Item = &EtaEnv> {
        self.0.iter()
    }
}

impl IntoIterator for EtaEnvs {
    type Item = EtaEnv;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtaEnv {
    pub ident: HandlerIdent,
    pub expr: CoroutineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImplTrait {
    pub constraints: Vec<TraitConstraint>,
    pub ident: TypeIdent,
    pub args: Vec<TypeIdent>,
    pub where_clause: Module,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Apply(Apply),
    ApplyInst(ApplyInst),
    ApplyEff(ApplyEff),
    InstIdent(InstIdent),
    Ident(Ident),
    HandlerIdent(HandlerIdent),
    Literal(Literal),
    Abstruction(Abstruction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Abstruction {
    pub arg: Option<Box<Expr>>,
    pub expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoroutineType {
    pub resume: TypeAbstructionExpr,
    pub ret: TypeAbstructionExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitDef {
    pub trait_constraints: Vec<TraitConstraint>,
    pub constructor: DataConstructor,
    pub where_clause: Module,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitConstraint {
    pub ident: TraitIdent,
    pub arg: ForallIdent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apply {
    pub abstruction: Abstruction,
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
    pub trait_constraints: Vec<TraitConstraint>,
    pub eta_envs: EtaEnvs,
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
    Constructor(DataConstructor),
    Tuple(usize, Vec<TypeAbstructionExpr>),
    Coroutine(Box<CoroutineType>),
    // Abstruction
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
pub struct DataAssign {
    pub modifier: Option<DataModifier>,
    pub constraints: Vec<TraitConstraint>,
    pub ident: TypeIdent,
    pub args: Vec<TypeIdent>,
    pub expr: DataExpr,
}

impl DataAssign {
    pub fn is_structual(&self) -> bool {
        match self.modifier {
            Some(DataModifier::Structual) => true,
            Some(DataModifier::Nominal) => false,
            None => true,
        }
    }

    pub fn is_nominal(&self) -> bool {
        !self.is_structual()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataModifier {
    Nominal,
    Structual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataExpr {
    Or(Box<DataExpr>, Box<DataExpr>),
    Value(DataValue),
}

impl DataExpr {
    pub fn iter(&self) -> impl Iterator<Item = &DataValue> {
        match self {
            DataExpr::Or(lhs, rhs) => lhs.iter().chain(rhs.iter()).collect::<Vec<_>>().into_iter(),
            DataExpr::Value(v) => vec![v].into_iter(),
        }
    }
}

impl IntoIterator for DataExpr {
    type Item = DataValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            DataExpr::Or(lhs, rhs) => lhs
                .into_iter()
                .chain(rhs.into_iter())
                .collect::<Vec<_>>()
                .into_iter(),
            DataExpr::Value(v) => vec![v].into_iter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataValue {
    Constructor(DataConstructor),
    Unit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataConstructor {
    pub modifier: Option<DataModifier>,
    pub ident: TypeIdent,
    pub args: Vec<TypeLiteral>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignArgs {
    pub patterns: Vec<PatternExpr>,
}

impl AssignArgs {
    pub fn iter(&self) -> impl Iterator<Item = &PatternExpr> {
        self.patterns.iter()
    }
}

impl IntoIterator for AssignArgs {
    type Item = PatternExpr;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.patterns.into_iter()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatternExpr {
    Or(Box<PatternExpr>, Box<PatternExpr>),
    Literal(Literal),
    Bind(Ident),
    ListHead(),
    Constructor(DataConstructor),
    Tuple(usize, Vec<PatternExpr>),
    Any,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignDef {
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

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum TypeIdent {
    ForallIdent(ForallIdent),
    ExistsIdent(ExistsIdent),
}

impl Display for TypeIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeIdent::ForallIdent(ident) => write!(f, "forall {}.", ident.0),
            TypeIdent::ExistsIdent(ident) => write!(f, "exists {}.", ident.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ForallIdent(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ExistsIdent(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct HandlerIdent(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct InstIdent(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct TraitIdent(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Char(char),
    Text(String),
    Int(isize),
    Unit,
    Array(Vec<Expr>),
    Tuple(usize, Vec<Expr>),
}
