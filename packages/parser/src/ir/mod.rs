use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    ops::Deref,
    rc::Rc,
};

use crate::ast::{self};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Module {
    pub values: Vec<Assign>,
    pub types: Vec<(Ident, TypeExpr)>,
    pub handlers: Vec<Handler>,
    pub insts: Vec<(Ident, TypeExpr)>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Expr(Rc<Expr>),
    Function(Rc<Assign>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    pub ident: Ident,
    pub ident_name: Ident,
    pub expr: Rc<Expr>,
    pub where_clause: Module,
    pub pat: Option<Rc<PatternExpr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Abstruction {
    pub ident: Ident,
    pub expr: Rc<Expr>,
    pub context: Rc<ContextStack>,
}

impl Abstruction {
    pub fn apply(&self, expr: Rc<Expr>) -> Assign {
        todo!()
        // let mut applied = self.applied.clone();
        // applied.push(expr.clone());

        // Self {
        //     ident: self.ident.apply_new(),
        //     ident_name: self.ident_name.clone(),
        //     expr: self.expr.clone(),
        //     applied,
        // }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeExpr {
    constraints: Vec<(Ident, TraitConstraint)>,
    handlers: Vec<HandlerTypeExpr>,
    type_abstruction: TypeAbstruction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitConstraint {}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerTypeExpr {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAbstruction {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Handler {}

impl From<ast::Module> for Module {
    fn from(value: ast::Module) -> Self {
        value
            .statements
            .into_iter()
            .fold(Self::default(), |mut acc, item| {
                match item {
                    ast::Statement::Assign(assign) => acc.values.push(assign.into()),
                    ast::Statement::TraitDef(_) => todo!(),
                    ast::Statement::AssignDef(_) => todo!(),
                    ast::Statement::InstDef(_) => todo!(),
                    ast::Statement::HandlerDef(_) => todo!(),
                    ast::Statement::HandlerAssign(_) => todo!(),
                    ast::Statement::LineComment(_) => {}
                };

                acc
            })
    }
}

impl From<ast::Assign> for Assign {
    fn from(value: ast::Assign) -> Self {
        let is_normal_form = value.args.patterns.len() == 0;
        let mut args = value.args.patterns;
        let pat = args.pop();

        Self {
            ident: format!("{}_{}", value.ident.0, seq_gen()).into(),
            ident_name: value.ident.into(),
            expr: Rc::new(value.expr.into()),
            pat: match is_normal_form {
                true => None,
                false => pat.map(|v| v.into()).map(Rc::new),
            },
            where_clause: value.where_clause.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternExpr {}

impl From<ast::PatternExpr> for PatternExpr {
    fn from(value: ast::PatternExpr) -> Self {
        match value {
            ast::PatternExpr::Or(_, _) => todo!(),
            ast::PatternExpr::Literal(_) => todo!(),
            ast::PatternExpr::Bind(_) => todo!(),
            ast::PatternExpr::ListHead() => todo!(),
            ast::PatternExpr::Tuple(_, _) => todo!(),
            ast::PatternExpr::Any => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Serialize, Deserialize, Hash)]
pub struct Ident {
    pub ident: String,
}

impl Ident {
    pub fn apply_new(&self) -> Ident {
        format!("{}__{}", &self.ident, seq_gen()).into()
    }
}

impl From<ast::Ident> for Ident {
    fn from(value: ast::Ident) -> Self {
        value.0.into()
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
        Self { ident: value }
    }
}

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self {
            ident: value.to_string().into(),
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Apply(Apply),
    ApplyEmbedded(ApplyInst),
    Reference(Ident),
    Pattern(Pattern),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub values: HashMap<Ident, Rc<Expr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextStack(VecDeque<Context>);

impl Deref for Context {
    type Target = HashMap<Ident, Rc<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl Expr {
    fn is_normal_form(&self, context: &Context) -> bool {
        match self {
            Expr::Literal(_) => true,
            Expr::ApplyEmbedded(_) => false,
            Expr::Apply(_) => false,
            Expr::Reference(ident) => {
                let value = context.get(ident).unwrap();
                value.is_normal_form(context)
            }
            Expr::Pattern(_) => false,
        }
    }

    fn eval(&self, context: &Context) -> Rc<Expr> {
        match self {
            Expr::Literal(value) => Rc::new(Expr::Literal(value.clone())),
            Expr::Apply(_) => todo!(),
            Expr::ApplyEmbedded(_) => todo!(),
            Expr::Reference(ident) => context
                .get(ident)
                .map(|item| match Self::is_normal_form(item, context) {
                    true => item.clone(),
                    false => self.eval(context),
                })
                .unwrap(),
            Expr::Pattern(_) => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub expr: Rc<Expr>,
    pub cases: Vec<Rc<Expr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apply {
    pub ident: Ident,
    pub expr: Rc<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyInst {
    pub ident: Ident,
    pub expr: Rc<Expr>,
}

impl From<ast::Expr> for Expr {
    fn from(value: ast::Expr) -> Self {
        match value {
            ast::Expr::Literal(value) => Self::Literal(value.into()),
            ast::Expr::Apply(value) => Self::Apply(value.into()),
            ast::Expr::ApplyInst(value) => Self::ApplyEmbedded(value.into()),
            ast::Expr::Ident(value) => Self::Reference(value.into()),
            ast::Expr::ApplyEff(_) => todo!(),
            ast::Expr::InstIdent(_) => todo!(),
            ast::Expr::HandlerIdent(_) => todo!(),
            ast::Expr::Abstruction(_) => todo!(),
        }
    }
}

impl From<ast::Apply> for Apply {
    fn from(value: ast::Apply) -> Self {
        Self {
            ident: value.ident.into(),
            expr: Rc::new((*value.expr).into()),
        }
    }
}

impl From<ast::ApplyInst> for ApplyInst {
    fn from(value: ast::ApplyInst) -> Self {
        Self {
            ident: value.ident.into(),
            expr: Rc::new((*value.expr).into()),
        }
    }
}

impl Expr {
    pub fn as_literal(&self) -> Literal {
        match self {
            Expr::Literal(v) => v.clone(),
            Expr::Apply(_) => panic!(),
            Expr::ApplyEmbedded(_) => panic!(),
            Expr::Reference(_) => panic!(),
            Expr::Pattern(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Char(char),
    Text(String),
    Int(isize),
    Tuple(usize, Box<[Rc<Expr>]>),
    List(Vec<Rc<Expr>>),
}

impl From<ast::Literal> for Literal {
    fn from(value: ast::Literal) -> Self {
        match value {
            ast::Literal::Char(value) => Self::Char(value),
            ast::Literal::Text(value) => Self::Text(value),
            ast::Literal::Int(value) => Self::Int(value),
            ast::Literal::Unit => Self::Tuple(0, Box::new([])),
            ast::Literal::Array(_) => todo!(),
            ast::Literal::Tuple(_, _) => todo!(),
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

fn seq_gen() -> usize {
    static mut COUNT: usize = 0;
    unsafe {
        COUNT += 1;

        COUNT
    }
}
