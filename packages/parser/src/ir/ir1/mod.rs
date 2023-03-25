mod lattice;

use crate::ast;
use anyhow::anyhow;
use lattice::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolTable {
    pub type_symbols: HashMap<Ident, TypeAbstruction>,
    pub data_lattice: BoundedDataLattice,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Stmt {
    AssignDef(Ident, TypeAbstruction),
    DataAssign(DataLattice),
}

impl TryFrom<ast::Module> for SymbolTable {
    type Error = Vec<anyhow::Error>;

    fn try_from(module: ast::Module) -> Result<Self, Self::Error> {
        let mut errs: Vec<anyhow::Error> = Vec::new();

        let (stmts, err): (Vec<_>, Vec<_>) = module
            .statements
            .into_iter()
            .map(Stmt::try_from)
            .partition(|v| v.is_ok());

        let (type_symbols, data_lattices) =
            stmts
                .into_iter()
                .fold((Vec::new(), Vec::new()), |mut acc, stmt| {
                    match stmt {
                        Ok(Stmt::AssignDef(ident, expr)) => {
                            acc.0.push((ident, expr));
                        }
                        Ok(Stmt::DataAssign(lattice)) => {
                            acc.1.push(lattice);
                        }
                        Err(err) => {
                            errs.push(err);
                        }
                    }
                    acc
                });

        let type_symbols = type_symbols.into_iter().collect::<HashMap<_, _>>();
        let data_lattice =
            data_lattices
                .into_iter()
                .fold(BoundedDataLattice::default(), |mut acc, lattice| {
                    acc.append(lattice);
                    acc
                });

        errs.append(&mut err.into_iter().map(Result::unwrap_err).collect());

        if errs.len() > 0 {
            return Err(errs);
        }

        Ok(Self {
            type_symbols,
            data_lattice,
        })
    }
}

impl TryFrom<ast::Statement> for Stmt {
    type Error = anyhow::Error;

    fn try_from(stmt: ast::Statement) -> Result<Self, Self::Error> {
        match stmt {
            ast::Statement::AssignDef(abs) => match TypeAbstruction::try_from(abs.expr.expr) {
                Ok(e) => Ok(Stmt::AssignDef(Ident::from(abs.ident), e)),
                Err(e) => Err(e),
            },
            ast::Statement::DataAssign(data) => match DataLattice::try_from(data) {
                Ok(e) => Ok(Stmt::DataAssign(e)),
                Err(e) => Err(e),
            },
            _ => Err(anyhow!("Unexpected statement")),
        }
    }
}

impl TryFrom<ast::DataAssign> for DataLattice {
    type Error = anyhow::Error;

    fn try_from(data: ast::DataAssign) -> Result<Self, Self::Error> {
        if !data.is_valid_modifier() {
            return Err(anyhow!("Invalid modifier"));
        }

        let mut lattice = DataLattice::default();

        if data.is_nominal() {
            let value = match (data.ident, data.expr) {
                (ast::TypeIdent::ExistsIdent(ident), ast::DataExpr::Value(value)) => DataElement {
                    ident: ExistsIdent::from(ident),
                    value: DataValue::from(value),
                },
                _ => {
                    return Err(anyhow!("Invalid nominal data"));
                }
            };
            lattice.append(value);
        };

        Ok(lattice)
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
