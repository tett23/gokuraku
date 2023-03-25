use super::{ExistsIdent, TypeIdent};
use crate::ast;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundedDataLattice {
    top: Vec<Rc<DataLattice>>,
    botom: Vec<Rc<DataLattice>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataLattice {
    pub elements: Vec<DataElement>,
    pub union_elements: Option<Rc<Box<DataLattice>>>,
    pub intersection_elements: Option<Rc<Box<DataLattice>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataElement {
    pub ident: ExistsIdent,
    pub value: DataValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataValue {
    Context(TypeIdent, Vec<TypeIdent>),
    TypeIdent(TypeIdent),
    Unit,
}

impl Default for BoundedDataLattice {
    fn default() -> Self {
        let mut lattice = DataLattice::default();
        lattice.append(DataElement {
            ident: ExistsIdent("Unit".into()),
            value: DataValue::Unit,
        });

        let mut ret = Self {
            top: Vec::new(),
            botom: Vec::new(),
        };

        ret.append(lattice);

        ret
    }
}

impl BoundedDataLattice {
    pub fn append(&mut self, lattice: DataLattice) {
        let lattice = Rc::new(lattice);

        self.top.push(lattice.clone());
        self.botom.push(lattice.clone());
    }
}

impl DataLattice {
    pub fn append(&mut self, element: DataElement) {
        self.elements.push(element);
    }
}

impl From<ast::DataValue> for DataValue {
    fn from(value: ast::DataValue) -> Self {
        match value {
            ast::DataValue::Context(ident, args) => {
                let args = args.into_iter().map(|arg| arg.into()).collect();
                Self::Context(ident.into(), args)
            }
            ast::DataValue::TypeIdent(ident) => Self::TypeIdent(ident.into()),
            ast::DataValue::Unit => Self::Unit,
        }
    }
}
