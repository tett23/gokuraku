use super::symbol_table2::SymbolTable2;
use crate::ast;
use crate::ir::ir1::IR1;
use crate::ir::{
    DataExpr, Function, HandlerFunction, HandlerIdent, Ident, InstIdent, TypeAbstructionEnv,
    TypeIdent,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IR2 {
    pub ident_symbols: SymbolTable2<Ident, (Vec<Function<IR2>>, Option<TypeAbstructionEnv>)>,
    pub data_symbols: SymbolTable2<TypeIdent, DataExpr>,
    pub handler_symbols:
        SymbolTable2<HandlerIdent, (Vec<HandlerFunction<IR2>>, Option<TypeAbstructionEnv>)>,
    // TODO: traitはTypeAbstructionではない
    pub trait_symbols: SymbolTable2<TypeIdent, TypeAbstructionEnv>,
    pub inst_symbols: SymbolTable2<InstIdent, TypeAbstructionEnv>,
}

pub fn transform2(_ir1: IR1, ast: ast::Module) -> Result<IR2> {
    let (ir2, errs) =
        ast.into_iter()
            .fold((IR2::default(), Vec::new()), |(mut acc, mut errs), item| {
                match item {
                    ast::Statement::DataAssign(data_assign) => {
                        let ident = TypeIdent::from(data_assign.ident.clone());
                        let value = match DataExpr::try_from(data_assign){
                            Ok(value)=>value,
                            Err(err)=>{errs.push(err);return (acc,errs);},
                        };

                                acc.data_symbols.insert(ident, value);
                    }
                    ast::Statement::Assign(assign) => {
                        let ident = Ident::from(assign.ident.clone());
                        let value = Function::try_from(assign);

                        match value {
                            Ok(value) => {
                                match acc.ident_symbols.remove(&ident) {
                                    Some((mut abstractions, type_abstruction)) => {
                                        abstractions.push(value);
                                        acc.ident_symbols
                                            .insert(ident, (abstractions, type_abstruction));
                                    }
                                    None => {
                                        acc.ident_symbols.insert(ident, (vec![value], None));
                                    }
                                };
                            }
                            Err(err) => errs.push(err),
                        }
                    }
                    ast::Statement::AssignDef(assign_def) => {
                        let ident = Ident::from(assign_def.ident.clone());
                        let value = match TypeAbstructionEnv::try_from(assign_def){
                            Ok(value)=>value,
                            Err(err)=>{errs.push(err);return (acc,errs);},
                        };

                                match acc.ident_symbols.remove(&ident) {
                                    Some((abstractions, None)) => {
                                        acc.ident_symbols.insert(ident, (abstractions, Some(value)));
                                    },
                                    Some((_abstractions, Some(_))) => {
                                        unreachable!("Should not be possible to have multiple type abstractions for the same ident");
                                    },
                                    None => {
                                        acc.ident_symbols.insert(ident, (vec![], Some(value)));
                                    },
                                };
                    }
                    ast::Statement::HandlerDef(handler_def) => {
                        let ident = HandlerIdent::from(handler_def.ident.clone());
                        let value = match TypeAbstructionEnv::try_from(handler_def){
                            Ok(value)=>{
                                value
                            },
                            Err(err)=>{errs.push(err); return (acc, errs);},
                        };

acc.handler_symbols.find_or_default_mut(&ident).1 = Some(value);

                    }
                    ast::Statement::HandlerAssign(handler_assign) => {
                        let ident = HandlerIdent::from(handler_assign.ident.clone());
                        let value = match HandlerFunction::try_from(handler_assign){
                            Ok(value)=>value,
                            Err(err)=>{errs.push(err); return (acc, errs);},
                        };

                        acc.handler_symbols.find_or_default_mut(&ident).0.push(value);

                    }
                    ast::Statement::TraitDef(trait_def) => {
                        let ident = TypeIdent::from(trait_def.constructor.ident.clone());
                        let value = match TypeAbstructionEnv::try_from(trait_def){
                            Ok(value)=>{
                                value
                            },
                            Err(err)=>{errs.push(err); return (acc, errs);},
                        };

                        acc.trait_symbols.insert(ident, value);
                    }
                    ast::Statement::ImplTrait(_impl_trait) => {
                        // TODO: impl trait
                    }
                    ast::Statement::InstDef(inst_def) => {
                        let ident = InstIdent::from(inst_def.ident.clone());
                        match TypeAbstructionEnv::try_from(inst_def){
                            Ok(value)=>{
                                acc.inst_symbols.insert(ident, value);
                            },
                            Err(err)=>errs.push(err),
                        };

                    }
                    ast::Statement::LineComment(_) => {}
                };

                (acc, errs)
            });
    if !errs.is_empty() {
        return Err(anyhow!("Errors: {:?}", errs));
    }

    Ok(ir2)
}

// fn build_data_table(
//     symbols: SymbolTable1<TypeIdent>,
//     stmts: Vec<ast::DataAssign>,
// ) -> Result<BoundedDataLattice> {
//     let symbols = symbols
//         .into_iter()
//         .map(|(ident, _)| ident)
//         .collect::<VecDeque<_>>();

//     let mut lattice = DataLattice::default();
//     let a = build_data_lattice_from_symbols(symbols, stmts, &mut lattice);
//     let mut bounded = BoundedDataLattice::default();
//     bounded.union(lattice);

//     Ok(bounded)
// }

// fn build_data_lattice_from_symbols(
//     mut symbols: VecDeque<TypeIdent>,
//     stmts: Vec<ast::DataAssign>,
//     lattice: &mut DataLattice,
// ) -> Result<(VecDeque<TypeIdent>, Vec<ast::DataAssign>, &mut DataLattice)> {
//     let symbol = match symbols.pop_front() {
//         Some(sym) => sym,
//         None => return Ok((symbols, stmts, lattice)),
//     };

//     let (mut matched, rest): (Vec<ast::DataAssign>, Vec<ast::DataAssign>) = stmts
//         .into_iter()
//         .partition(|item| TypeIdent::from(item.ident.clone()) == symbol);
//     let def = match &matched.as_slice() {
//         &[] => return Err(anyhow!("No data assign for symbol {}", symbol)),
//         &[_] => matched.pop().unwrap(),
//         _ => return Err(anyhow!("Multiple data assign for symbol {}", symbol)),
//     };

//     dbg!(&def.expr);
//     match (def.modifier.into(), def.ident) {
//         (DataModifier::Nominal, ast::TypeIdent::ExistsIdent(ident)) => {
//             let mut l = DataLattice::default();
//             // TODO: ここで、型変数を束縛する
//             dbg!(build_data_lattice_from_expr(def.expr, lattice));
//             // l.element = DataTerm::Constructor(ident.into(), Vec::new());
//             // lattice.union(l);
//             todo!()
//         }
//         (DataModifier::Nominal, ast::TypeIdent::ForallIdent(_ident)) => {
//             // TODO: forallの場合は、型変数を束縛する
//         }
//         (DataModifier::Structual, ident) => {
//             dbg!("structual", &ident);
//             // TODO: 構造体とタプルの中身で新しい型が出現しないかチェックする
//         }
//     }

//     let l = build_data_lattice_from_expr(def.expr, lattice);
//     lattice.union(l);

//     //     match def.expr {
//     //         ast::DataExpr::Or(lhs, rhs) => {
//     //             let (symbols, rest, lattice) = build_data_lattice_from_symbols(symbols, rest, lattice)?;
//     //             let (symbols, rest, lattice) = build_data_lattice_from_symbols(symbols, rest, lattice)?;

//     // build_data_lattice_from_expr(lhs, lattice)

//     //             lattice.union(lhs);

//     //             build_data_lattice_from_symbols(symbols, rest, lattice)
//     //         }
//     //         ast::DataExpr::Value(value) => match value {
//     //             ast::DataValue::Unit => build_data_lattice_from_symbols(symbols, rest, lattice),
//     //             ast::DataValue::Constructor(ident, args) => {
//     //                 dbg!(&ident, &args);

//     //                 build_data_lattice_from_symbols(symbols, rest, lattice)
//     //             }
//     //         },
//     //     }

//     build_data_lattice_from_symbols(symbols, rest, lattice)
// }

// fn build_data_lattice_from_expr(expr: ast::DataExpr, lattice: &mut DataLattice) -> DataLattice {
//     match expr {
//         ast::DataExpr::Or(lhs, rhs) => {
//             let mut l = DataLattice::default();
//             l.union(build_data_lattice_from_expr(*lhs, lattice));
//             l.union(build_data_lattice_from_expr(*rhs, lattice));

//             l
//         }
//         ast::DataExpr::Value(value) => DataTerm::from(value).into(),
//     }
// }
