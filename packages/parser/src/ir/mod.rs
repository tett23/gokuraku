use serde::{Deserialize, Serialize};
use {
    crate::ast,
    ast::{Assign, Pds, TopLevel},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Module {
    pub values: Vec<Value>,
    pub types: Vec<Type>,
    pub handlers: Vec<Handler>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    ident: Ident,
    ident_name: Ident,
    arity: usize,
    expr: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Handler {}

impl From<Pds> for Module {
    fn from(Pds(pds): Pds) -> Self {
        pds.into_iter().fold(Module::default(), |mut acc, item| {
            match item {
                TopLevel::Assign(assign) => acc.values.push(assign.into()),
                TopLevel::AssignAnnotation(_) => {}
                TopLevel::LineComment(_) => todo!(),
                TopLevel::Environment() => todo!(),
            };

            acc
        })
    }
}

impl From<Assign> for Value {
    fn from(value: Assign) -> Self {
        Self {
            ident: format!("{}_{}", value.ident.0, seq_gen()).into(),
            ident_name: value.ident.into(),
            arity: value.args.patterns.len(),
            expr: Box::new(value.expr.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident {
    pub ident: String,
}

impl From<ast::Ident> for Ident {
    fn from(value: ast::Ident) -> Self {
        value.0.into()
    }
}

impl From<String> for Ident {
    fn from(value: String) -> Self {
        Self { ident: value }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Apply(Value, Vec<Expr>),
}

impl From<ast::Expr> for Expr {
    fn from(value: ast::Expr) -> Self {
        match value {
            ast::Expr::Apply(_ident, _expr) => {
                todo!()
            }
            ast::Expr::Literal(value) => Self::Literal(value.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Char(char),
    Text(String),
    Int(isize),
}

impl From<ast::Literal> for Literal {
    fn from(value: ast::Literal) -> Self {
        match value {
            ast::Literal::Char(value) => Self::Char(value),
            ast::Literal::Text(value) => Self::Text(value),
            ast::Literal::Int(value) => Self::Int(value),
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
