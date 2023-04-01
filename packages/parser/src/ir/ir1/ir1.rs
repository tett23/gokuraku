use super::symbol_table1::SymbolTable1;
use crate::{
    ast,
    ir::{HandlerIdent, Ident, InstIdent, TypeIdent},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IR1 {
    pub ident_symbols: SymbolTable1<Ident>,
    pub data_symbols: SymbolTable1<TypeIdent>,
    pub handler_symbols: SymbolTable1<HandlerIdent>,
    pub trait_symbols: SymbolTable1<TypeIdent>,
    pub inst_symbols: SymbolTable1<InstIdent>,
}

// TODO: detect cyclic dependency
pub fn transform1(ast: &ast::Module) -> IR1 {
    ast.iter().fold(IR1::default(), |mut acc, stmt| {
        match stmt {
            ast::Statement::DataAssign(ast::DataAssign { ident, .. }) => {
                acc.data_symbols.insert(ident.clone().into());
            }
            ast::Statement::Assign(ast::Assign { ident, .. }) => {
                acc.ident_symbols.insert(ident.clone().into());
            }
            ast::Statement::AssignDef(ast::AssignDef { ident, .. }) => {
                acc.ident_symbols.insert(ident.clone().into());
            }
            ast::Statement::HandlerDef(ast::HandlerDef { ident, .. }) => {
                acc.handler_symbols.insert(ident.clone().into());
            }
            ast::Statement::HandlerAssign(ast::HandlerAssign { ident, .. }) => {
                acc.handler_symbols.insert(ident.clone().into());
            }
            ast::Statement::TraitDef(ast::TraitDef {
                constructor: ast::Constructor { ident, .. },
                ..
            }) => {
                acc.trait_symbols.insert(ident.clone().into());
            }
            ast::Statement::ImplTrait(ast::ImplTrait { ident, .. }) => {
                acc.trait_symbols.insert(ident.clone().into());
            }
            ast::Statement::InstDef(ast::InstDef { ident, .. }) => {
                acc.inst_symbols.insert(ident.clone().into());
            }
            ast::Statement::LineComment(_) => {}
        };

        acc
    })
}
