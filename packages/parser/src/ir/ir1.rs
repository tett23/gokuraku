use crate::ast;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolTable {
    pub type_symbols: HashMap<Ident, TypeAbstruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeValue {
    Top,
    Bottom,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeTerm {
    Value(TypeValue),
    Abstruction(TypeAbstruction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAbstruction {
    arg: Option<Box<TypeTerm>>,
    expr: Box<TypeTerm>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TypeIdent {
    ForallIdent(ForallIdent),
    ExistsIdent(ExistsIdent),
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ForallIdent(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ExistsIdent(pub String);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Ident(pub String);

impl TryFrom<ast::Module> for SymbolTable {
    type Error = Vec<anyhow::Error>;

    fn try_from(module: ast::Module) -> Result<Self, Self::Error> {
        let (type_symbols, err): (Vec<_>, Vec<_>) = module
            .statements
            .into_iter()
            .map(|stmt| match stmt {
                ast::Statement::AssignDef(abs) => match TypeAbstruction::try_from(abs.expr.expr) {
                    Ok(e) => Ok((Ident::from(abs.ident), e)),
                    Err(e) => Err(e),
                },
                _ => Err(anyhow!("Unexpected statement")),
            })
            .partition(|v| v.is_ok());
        if err.len() > 0 {
            return Err(err.into_iter().map(|v| v.unwrap_err()).collect());
        }

        let type_symbols = type_symbols
            .into_iter()
            .map(|v| v.unwrap())
            .collect::<HashMap<_, _>>();

        Ok(Self { type_symbols })
    }
}

impl TryFrom<ast::TypeAbstructionExpr> for TypeAbstruction {
    type Error = anyhow::Error;

    fn try_from(expr: ast::TypeAbstructionExpr) -> Result<Self, Self::Error> {
        match expr {
            ast::TypeAbstructionExpr::Arrow(lhs, rhs) => {
                let lhs = TypeTerm::try_from(*lhs)?;
                let rhs = TypeTerm::try_from(*rhs)?;

                Ok(Self {
                    arg: Some(Box::new(lhs)),
                    expr: Box::new(rhs),
                })
            }
            ast::TypeAbstructionExpr::Literal(_literal) => Ok(TypeAbstruction {
                arg: None,
                expr: Box::new(TypeTerm::Value(TypeValue::Bottom)),
            }),
        }
    }
}

impl TryFrom<ast::TypeAbstructionExpr> for TypeTerm {
    type Error = anyhow::Error;

    fn try_from(expr: ast::TypeAbstructionExpr) -> Result<Self, Self::Error> {
        match expr {
            ast::TypeAbstructionExpr::Arrow(lhs, rhs) => {
                Ok(TypeTerm::Abstruction(TypeAbstruction {
                    arg: Some(Box::new(TypeTerm::try_from(*lhs)?)),
                    expr: Box::new(TypeTerm::try_from(*rhs)?),
                }))
            }
            ast::TypeAbstructionExpr::Literal(_literal) => Ok(TypeTerm::Value(TypeValue::Bottom)),
        }
    }
}

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
