pub mod ir1;
pub mod ir2;
mod lattice;
mod seq_gen;

use crate::ast::{self};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Function<T: Default> {
    pub ident: Ident,
    pub expr: Abstruction,
    pub where_clause: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerFunction<T: Default> {
    pub ident: HandlerIdent,
    pub expr: Abstruction,
    pub where_clause: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Abstruction {
    pub arg: Option<PatternExpr>,
    pub expr: Expr,
}

impl From<ast::Abstruction> for Abstruction {
    fn from(value: ast::Abstruction) -> Self {
        Self {
            arg: value.arg.map(|arg| PatternExpr::from(*arg)),
            expr: Expr::from(*value.expr),
        }
    }
}

impl<T: Default> TryFrom<ast::Assign> for Function<T> {
    type Error = anyhow::Error;

    fn try_from(value: ast::Assign) -> Result<Self, Self::Error> {
        let expr = build_abstruction(value.args.into_iter().rev().collect::<Vec<_>>(), value.expr);

        Ok(Self {
            ident: value.ident.into(),
            expr,
            where_clause: T::default(),
        })
    }
}

fn build_abstruction(mut args: Vec<ast::PatternExpr>, expr: ast::Expr) -> Abstruction {
    let pat = match args.pop() {
        Some(pat) => pat,
        None => {
            return Abstruction {
                arg: None,
                expr: Expr::try_from(expr).unwrap(),
            }
        }
    };

    Abstruction {
        arg: Some(PatternExpr::from(pat)),
        expr: Expr::Abstruction(Box::new(build_abstruction(args, expr))),
    }
}

impl<T: Default> TryFrom<ast::HandlerAssign> for HandlerFunction<T> {
    type Error = anyhow::Error;

    fn try_from(value: ast::HandlerAssign) -> Result<Self, Self::Error> {
        let expr = build_abstruction(value.args.into_iter().rev().collect::<Vec<_>>(), value.expr);

        Ok(Self {
            ident: value.ident.into(),
            expr,
            where_clause: T::default(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeExpr {
    pub constraints: Vec<TraitConstraint>,
    pub eta_env: Vec<CoroutineType>,
    pub type_abstruction: TypeAbstruction,
}

impl TryFrom<ast::TypeExpr> for TypeExpr {
    type Error = anyhow::Error;

    fn try_from(value: ast::TypeExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            constraints: value
                .trait_constraints
                .into_iter()
                .map(TraitConstraint::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            // TDOO
            eta_env: Vec::new(),
            type_abstruction: TypeAbstruction::try_from(value.expr)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitConstraint {
    pub ident: TraitIdent,
    pub arg: ForallIdent,
}

impl From<ast::TraitConstraint> for TraitConstraint {
    fn from(value: ast::TraitConstraint) -> Self {
        Self {
            ident: value.ident.into(),
            arg: value.arg.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerConstraint {
    pub ident: HandlerIdent,
    pub expr: CoroutineType,
}

impl From<ast::EtaEnv> for HandlerConstraint {
    fn from(value: ast::EtaEnv) -> Self {
        Self {
            ident: value.ident.into(),
            expr: value.expr.into(),
        }
    }
}

impl From<ast::CoroutineType> for CoroutineType {
    fn from(value: ast::CoroutineType) -> Self {
        Self {
            resume: value.resume.into(),
            ret: value.ret.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoroutineType {
    resume: TypeAbstruction,
    ret: TypeAbstruction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatternExpr {
    Or(Box<PatternExpr>, Box<PatternExpr>),
    Literal(Literal),
    Bind(Ident),
    Constructor(DataConstructor),
    ListHead(),
    Tuple(usize, Vec<PatternExpr>),
    Any,
}

impl From<ast::PatternExpr> for PatternExpr {
    fn from(value: ast::PatternExpr) -> Self {
        match value {
            ast::PatternExpr::Or(lhs, rhs) => {
                Self::Or(Box::new((*lhs).into()), Box::new((*rhs).into()))
            }
            ast::PatternExpr::Literal(lit) => Self::Literal(lit.into()),
            ast::PatternExpr::Bind(ident) => Self::Bind(ident.into()),
            ast::PatternExpr::ListHead() => {
                todo!()
            }
            ast::PatternExpr::Tuple(size, exprs) => {
                Self::Tuple(size, exprs.into_iter().map(PatternExpr::from).collect())
            }
            ast::PatternExpr::Constructor(constructor) => Self::Constructor(constructor.into()),
            ast::PatternExpr::Any => Self::Any,
        }
    }
}

impl From<ast::Expr> for PatternExpr {
    fn from(value: ast::Expr) -> Self {
        match value {
            ast::Expr::Literal(lit) => Self::Literal(lit.into()),
            ast::Expr::Ident(ident) => Self::Bind(ident.into()),
            _ => unreachable!("{:?}", value),
        }
    }
}

// あやしい
impl From<ast::InstIdent> for Ident {
    fn from(value: ast::InstIdent) -> Self {
        value.0.into()
    }
}

impl From<String> for Ident {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Abstruction(Box<Abstruction>),
    Apply(Apply),
    ApplyEmbedded(ApplyInst),
    ApplyEff(ApplyEff),
    Reference(Ident),
    ReferenceInst(InstIdent),
    ReferenceHandler(HandlerIdent),
    Pattern(Pattern),
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Context {
//     pub values: HashMap<Ident, Rc<Expr>>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ContextStack(VecDeque<Context>);

// impl Deref for Context {
//     type Target = HashMap<Ident, Rc<Expr>>;

//     fn deref(&self) -> &Self::Target {
//         &self.values
//     }
// }

// impl Expr {
//     fn is_normal_form(&self, context: &Context) -> bool {
//         match self {
//             Expr::Literal(_) => true,
//             Expr::ApplyEmbedded(_) => false,
//             Expr::Apply(_) => false,
//             Expr::Reference(ident) => {
//                 let value = context.get(ident).unwrap();
//                 value.is_normal_form(context)
//             }
//             Expr::Pattern(_) => false,
//         }
//     }

//     fn eval(&self, context: &Context) -> Rc<Expr> {
//         match self {
//             Expr::Literal(value) => Rc::new(Expr::Literal(value.clone())),
//             Expr::Apply(_) => todo!(),
//             Expr::ApplyEmbedded(_) => todo!(),
//             Expr::Reference(ident) => context
//                 .get(ident)
//                 .map(|item| match Self::is_normal_form(item, context) {
//                     true => item.clone(),
//                     false => self.eval(context),
//                 })
//                 .unwrap(),
//             Expr::Pattern(_) => todo!(),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAbstructionEnv {
    trait_constraints: Vec<TraitConstraint>,
    handler_constraints: Vec<HandlerConstraint>,
    abstruction: TypeAbstruction,
}

impl TryFrom<ast::AssignDef> for TypeAbstructionEnv {
    type Error = anyhow::Error;

    fn try_from(value: ast::AssignDef) -> Result<Self, Self::Error> {
        let trait_constraints = value
            .expr
            .trait_constraints
            .into_iter()
            .map(|item| item.into())
            .collect();
        let handler_constraints = value
            .expr
            .eta_envs
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let abstruction = value.expr.expr.try_into()?;

        Ok(Self {
            trait_constraints,
            handler_constraints,
            abstruction,
        })
    }
}

impl TryFrom<ast::HandlerDef> for TypeAbstructionEnv {
    type Error = anyhow::Error;

    fn try_from(value: ast::HandlerDef) -> Result<Self, Self::Error> {
        let trait_constraints = value
            .expr
            .trait_constraints
            .into_iter()
            .map(|item| item.into())
            .collect();
        let handler_constraints = value
            .expr
            .eta_envs
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let abstruction = value.expr.expr.try_into()?;

        Ok(Self {
            trait_constraints,
            handler_constraints,
            abstruction,
        })
    }
}

impl TryFrom<ast::InstDef> for TypeAbstructionEnv {
    type Error = anyhow::Error;

    fn try_from(value: ast::InstDef) -> Result<Self, Self::Error> {
        let trait_constraints = value
            .expr
            .trait_constraints
            .into_iter()
            .map(|item| item.into())
            .collect();
        let handler_constraints = value
            .expr
            .eta_envs
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let abstruction = value.expr.expr.try_into()?;

        Ok(Self {
            trait_constraints,
            handler_constraints,
            abstruction,
        })
    }
}

impl TryFrom<ast::TraitDef> for TypeAbstructionEnv {
    type Error = anyhow::Error;

    fn try_from(value: ast::TraitDef) -> Result<Self, Self::Error> {
        let trait_constraints = value
            .trait_constraints
            .into_iter()
            .map(|item| item.into())
            .collect();
        let handler_constraints = Vec::new();
        let abstruction = TypeAbstruction::Term(TypeValue::Unknown);

        Ok(Self {
            trait_constraints,
            handler_constraints,
            abstruction,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub expr: Rc<Expr>,
    pub cases: Vec<Rc<Expr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apply {
    pub abstruction: Box<Abstruction>,
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

impl From<ast::Expr> for Expr {
    fn from(value: ast::Expr) -> Self {
        match value {
            ast::Expr::Literal(value) => Self::Literal(value.into()),
            ast::Expr::Apply(value) => Self::Apply(value.into()),
            ast::Expr::ApplyInst(value) => Self::ApplyEmbedded(value.into()),
            ast::Expr::Ident(value) => Self::Reference(value.into()),
            ast::Expr::ApplyEff(value) => Self::ApplyEff(value.into()),
            ast::Expr::InstIdent(value) => Self::ReferenceInst(value.into()),
            ast::Expr::HandlerIdent(value) => Self::ReferenceHandler(value.into()),
            ast::Expr::Abstruction(value) => {
                dbg!(&value);
                // Self::Abstruction(Box::new());
                todo!()
            }
        }
    }
}

impl From<ast::Apply> for Apply {
    fn from(value: ast::Apply) -> Self {
        Self {
            abstruction: Box::new(value.abstruction.into()),
            expr: Box::new((*value.expr).into()),
        }
    }
}

impl From<ast::ApplyInst> for ApplyInst {
    fn from(value: ast::ApplyInst) -> Self {
        Self {
            ident: value.ident.into(),
            expr: Box::new((*value.expr).into()),
        }
    }
}

impl From<ast::ApplyEff> for ApplyEff {
    fn from(value: ast::ApplyEff) -> Self {
        Self {
            ident: value.ident.into(),
            expr: Box::new((*value.expr).into()),
        }
    }
}
// impl Expr {
//     pub fn as_literal(&self) -> Literal {
//         match self {
//             Expr::Literal(v) => v.clone(),
//             Expr::Apply(_) => panic!(),
//             Expr::ApplyEmbedded(_) => panic!(),
//             Expr::Reference(_) => panic!(),
//             Expr::Pattern(_) => todo!(),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Char(char),
    Text(String),
    Int(isize),
    Tuple(usize, Vec<Expr>),
    List(Vec<Expr>),
}

impl From<ast::Literal> for Literal {
    fn from(value: ast::Literal) -> Self {
        match value {
            ast::Literal::Char(value) => Self::Char(value),
            ast::Literal::Text(value) => Self::Text(value),
            ast::Literal::Int(value) => Self::Int(value),
            ast::Literal::Unit => Self::Tuple(0, Vec::new()),
            ast::Literal::Array(_) => todo!(),
            ast::Literal::Tuple(size, elems) => {
                Self::Tuple(size, elems.into_iter().map(|item| item.into()).collect())
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Char(value) => write!(f, "{value}"),
            Literal::Text(value) => write!(f, "{value}"),
            Literal::Int(value) => write!(f, "{value}"),
            Literal::Tuple(_, _) => todo!(),
            Literal::List(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ForallIdent(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ExistsIdent(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TypeIdent {
    ForallIdent(ForallIdent),
    ExistsIdent(ExistsIdent),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct InstIdent(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct TraitIdent(pub String);

impl From<ast::TraitIdent> for TraitIdent {
    fn from(ident: ast::TraitIdent) -> Self {
        TraitIdent(ident.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct HandlerIdent(pub String);

impl From<ast::TypeIdent> for TypeIdent {
    fn from(ident: ast::TypeIdent) -> Self {
        match ident {
            ast::TypeIdent::ForallIdent(ident) => TypeIdent::ForallIdent(ident.into()),
            ast::TypeIdent::ExistsIdent(ident) => TypeIdent::ExistsIdent(ident.into()),
        }
    }
}

impl From<ast::ForallIdent> for ForallIdent {
    fn from(ident: ast::ForallIdent) -> Self {
        ForallIdent(ident.0)
    }
}

impl From<ast::ExistsIdent> for ExistsIdent {
    fn from(ident: ast::ExistsIdent) -> Self {
        ExistsIdent(ident.0)
    }
}

impl From<ast::Ident> for Ident {
    fn from(ident: ast::Ident) -> Self {
        Ident(ident.0)
    }
}

impl TryFrom<ast::TypeIdent> for ExistsIdent {
    type Error = anyhow::Error;

    fn try_from(ident: ast::TypeIdent) -> Result<Self, Self::Error> {
        match ident {
            ast::TypeIdent::ExistsIdent(ident) => Ok(ident.into()),
            ast::TypeIdent::ForallIdent(ident) => {
                Err(anyhow!("Unexpected forall ident: {}", ident.0))
            }
        }
    }
}

impl From<ast::InstIdent> for InstIdent {
    fn from(ident: ast::InstIdent) -> Self {
        InstIdent(ident.0)
    }
}

impl From<ast::HandlerIdent> for HandlerIdent {
    fn from(ident: ast::HandlerIdent) -> Self {
        HandlerIdent(ident.0)
    }
}

impl From<&str> for ExistsIdent {
    fn from(value: &str) -> Self {
        ExistsIdent(value.to_string())
    }
}

impl Display for TypeIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeIdent::ForallIdent(ident) => write!(f, "forall {}.", ident.0),
            TypeIdent::ExistsIdent(ident) => write!(f, "exists {}.", ident.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeAbstruction {
    Arrow(Box<TypeAbstruction>, Box<TypeAbstruction>),
    Term(TypeValue),
}

impl From<ast::TypeAbstructionExpr> for TypeAbstruction {
    fn from(expr: ast::TypeAbstructionExpr) -> Self {
        match expr {
            ast::TypeAbstructionExpr::Arrow(lhs, rhs) => {
                let lhs = TypeAbstruction::from(*lhs);
                let rhs = TypeAbstruction::from(*rhs);

                Self::Arrow(Box::new(lhs), Box::new(rhs))
            }
            ast::TypeAbstructionExpr::Literal(literal) => TypeAbstruction::Term(literal.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeValue {
    Top,
    Bottom,
    Unknown,
    Array(Box<TypeAbstruction>),
    Constructor(DataConstructor),
    Tuple(usize, Vec<TypeAbstruction>),
    Ident(TypeIdent),
    Coroutine(Box<CoroutineType>),
}

impl From<ast::TypeLiteral> for TypeValue {
    fn from(literal: ast::TypeLiteral) -> Self {
        match literal {
            ast::TypeLiteral::Top => TypeValue::Top,
            ast::TypeLiteral::Bottom => TypeValue::Bottom,
            ast::TypeLiteral::Array(expr) => {
                TypeValue::Array(Box::new(TypeAbstruction::try_from(*expr).unwrap()))
            }
            ast::TypeLiteral::Constructor(constructor) => {
                TypeValue::Constructor(constructor.into())
            }
            ast::TypeLiteral::Tuple(size, exprs) => TypeValue::Tuple(
                size,
                exprs
                    .into_iter()
                    .map(|expr| TypeAbstruction::try_from(expr).unwrap())
                    .collect(),
            ),
            ast::TypeLiteral::Coroutine(ident) => TypeValue::Coroutine(Box::new((*ident).into())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataDef {
    Structual(StructualDataDef),
    Nominal(NominalDataDef),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructualDataDef {
    pub args: Vec<TypeIdent>,
    pub expr: DataExpr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NominalDataDef {
    pub ident: TypeIdent,
    pub args: Vec<TypeIdent>,
    pub expr: DataExpr,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataModifier {
    Structual,
    Nominal,
}

impl From<Option<ast::DataModifier>> for DataModifier {
    fn from(modifier: Option<ast::DataModifier>) -> Self {
        match modifier {
            Some(ast::DataModifier::Structual) => DataModifier::Structual,
            Some(ast::DataModifier::Nominal) => DataModifier::Nominal,
            None => DataModifier::Structual,
        }
    }
}

impl From<ast::DataModifier> for DataModifier {
    fn from(modifier: ast::DataModifier) -> Self {
        match modifier {
            ast::DataModifier::Structual => DataModifier::Structual,
            ast::DataModifier::Nominal => DataModifier::Nominal,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataExpr {
    Or(Box<DataExpr>, Box<DataExpr>),
    Term(DataTerm),
}

impl TryFrom<ast::DataExpr> for DataExpr {
    type Error = anyhow::Error;

    fn try_from(value: ast::DataExpr) -> Result<Self, Self::Error> {
        match value {
            ast::DataExpr::Or(lhs, rhs) => {
                let lhs = DataExpr::try_from(*lhs)?;
                let rhs = DataExpr::try_from(*rhs)?;

                Ok(Self::Or(Box::new(lhs), Box::new(rhs)))
            }
            ast::DataExpr::Value(term) => Ok(DataExpr::Term(term.into())),
        }
    }
}

impl TryFrom<ast::DataAssign> for DataExpr {
    type Error = anyhow::Error;

    fn try_from(value: ast::DataAssign) -> Result<Self, Self::Error> {
        value.expr.try_into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataTerm {
    Top,
    Bottom,
    Unknown,
    Unit,
    Parameter(ForallIdent),
    Ident(ExistsIdent),
    Tuple(usize, Vec<DataTerm>),
    Abstruction(Box<TypeAbstruction>),
    Constructor(DataConstructor),
    Coroutine(Box<CoroutineType>),
}

impl From<ast::DataValue> for DataTerm {
    fn from(value: ast::DataValue) -> Self {
        match value {
            ast::DataValue::Constructor(constructor) => DataTerm::Constructor(constructor.into()),
            ast::DataValue::Unit => DataTerm::Unit,
        }
    }
}

impl From<ast::TypeLiteral> for DataTerm {
    fn from(value: ast::TypeLiteral) -> Self {
        match value {
            ast::TypeLiteral::Top => DataTerm::Top,
            ast::TypeLiteral::Bottom => DataTerm::Bottom,
            ast::TypeLiteral::Array(_) => {
                todo!()
            }
            ast::TypeLiteral::Constructor(constructor) => DataTerm::Constructor(constructor.into()),
            ast::TypeLiteral::Tuple(size, elems) => DataTerm::Tuple(
                size,
                elems.into_iter().map(|p| p.into()).collect::<Vec<_>>(),
            ),
            ast::TypeLiteral::Coroutine(coroutine) => {
                DataTerm::Coroutine(Box::new((*coroutine).into()))
            }
        }
    }
}

impl From<ast::TypeAbstructionExpr> for DataTerm {
    fn from(value: ast::TypeAbstructionExpr) -> Self {
        DataTerm::Abstruction(Box::new(value.into()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataConstructor(DataModifier, TypeIdent, Vec<DataTerm>);

impl From<ast::DataConstructor> for DataConstructor {
    fn from(value: ast::DataConstructor) -> Self {
        Self(
            value.modifier.into(),
            value.ident.into(),
            value.args.into_iter().map(|p| p.into()).collect::<Vec<_>>(),
        )
    }
}
