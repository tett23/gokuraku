use super::super::{DataModifier, DataTerm, ExistsIdent, TypeIdent};
use crate::ast;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundedDataLattice {
    top: Vec<Rc<RefCell<DataLattice>>>,
    botom: Vec<Rc<RefCell<DataLattice>>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataLattice {
    pub element: DataTerm,
    pub join_element: Option<Rc<RefCell<DataLattice>>>,
    pub meet_elements: Vec<Rc<RefCell<DataLattice>>>,
}

impl Default for DataLattice {
    fn default() -> Self {
        Self {
            element: DataTerm::Unit,
            join_element: None,
            meet_elements: Vec::new(),
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub enum DataElement {
//     // Nominal(ExistsIdent, DataValue),
//     Constructor(ExistsIdent, Vec<DataValue>),
//     Unit,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub enum DataValue {
//     Context(TypeIdent, Vec<TypeIdent>),
//     TypeIdent(TypeIdent),
//     Unit,
//     Unknown,
//     Top,
//     Bottom,
//     Parameter,
// }

impl Default for BoundedDataLattice {
    fn default() -> Self {
        let lattice = DataLattice::default();

        let mut ret = Self {
            top: Vec::new(),
            botom: Vec::new(),
        };

        ret.union(lattice);

        ret
    }
}

trait Lattice {
    type Elem;

    fn is_disjoint(&self, lattice: &Self) -> bool;
    fn union(&mut self, lattice: Self);
    fn intersection(&mut self, lattice: Self);
}

impl BoundedDataLattice {
    pub fn is_disjoint(&self, lattice: &DataLattice) -> bool {
        !self
            .top
            .iter()
            .any(|l| l.as_ref().borrow().is_disjoint(lattice))
    }

    pub fn union(&mut self, lattice: DataLattice) {
        if !self.is_disjoint(&lattice) {
            return;
        }

        let elem = Rc::new(RefCell::new(lattice));

        self.top.push(elem.clone());
        self.botom.push(elem.clone());
    }

    pub fn remove(&mut self, ident: &ExistsIdent) -> Option<Rc<RefCell<DataLattice>>> {
        let idx = self
            .top
            .iter()
            .position(|l| DataLattice::lookup_from_top(l.clone(), ident).is_some());

        match idx {
            Some(idx) => {
                let lattice = self.top.remove(idx);
                self.botom.remove(idx);

                Some(lattice)
            }
            None => self
                .top
                .iter()
                .find_map(|l| DataLattice::remove(l.clone(), ident)),
        }
    }

    pub fn lookup_from_top(&self, ident: &ExistsIdent) -> Option<Rc<RefCell<DataLattice>>> {
        self.top
            .iter()
            .find_map(|l| DataLattice::lookup_from_top(l.clone(), ident))
    }
}

impl PartialOrd for DataLattice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.is_disjoint(other) {
            return None;
        }

        match self.element {
            DataTerm::Top => Some(std::cmp::Ordering::Greater),
            DataTerm::Bottom => Some(std::cmp::Ordering::Less),
            _ => todo!(),
        }
    }
}

impl Ord for DataLattice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl DataLattice {
    pub fn top() -> Self {
        Self {
            element: DataTerm::Top,
            join_element: None,
            meet_elements: Vec::new(),
        }
    }

    pub fn is_disjoint(&self, lattice: &DataLattice) -> bool {
        // PartialOrd::partial_cmpで比較してNoneのとき互いに素

        if self != lattice {
            return true;
        } else {
            self.meet_elements
                .iter()
                .all(|l| l.as_ref().borrow().is_disjoint(lattice))
        }
    }

    pub fn union(&mut self, mut lattice: DataLattice) {
        // dbg!("union", self.is_disjoint(&lattice), &self, &lattice);
        if !self.is_disjoint(&lattice) {
            return;
        }

        if self != &lattice {
            self.meet_elements.push(Rc::new(RefCell::new(lattice)));
            return;
        }

        let min = self.min(&mut lattice);
        let max = (&mut lattice).max(self);

        // self.meet_elements
        //     .iter()
        //     .for_each(|lhs| lhs.as_ref().borrow_mut().union(lattice));

        // self.meet_elements.push(Rc::new(RefCell::new(lattice)));
    }

    pub fn intersection(&mut self, lattice: DataLattice) {
        if !self.is_disjoint(&lattice) {
            return;
        }

        self.meet_elements = Vec::new();
    }

    pub fn remove(
        l: Rc<RefCell<DataLattice>>,
        ident: &ExistsIdent,
    ) -> Option<Rc<RefCell<DataLattice>>> {
        let idx = l
            .as_ref()
            .borrow()
            .meet_elements
            .iter()
            .position(|l| Self::lookup_from_top(l.clone(), ident).is_some());

        if let Some(idx) = idx {
            Some(l.as_ref().borrow_mut().meet_elements.remove(idx))
        } else {
            None
        }
    }

    pub fn lookup_from_top(
        lattice: Rc<RefCell<DataLattice>>,
        ident: &ExistsIdent,
    ) -> Option<Rc<RefCell<DataLattice>>> {
        match lattice.as_ref().borrow().element {
            // TODO: パラメータのチェック
            DataTerm::Constructor(ref _modifier, ref i, _) if i == ident => {
                return Some(lattice.clone())
            }
            _ => {}
        };

        lattice
            .as_ref()
            .borrow()
            .meet_elements
            .iter()
            .find_map(|l| Self::lookup_from_top(l.clone(), ident))
    }
}

impl From<DataTerm> for DataLattice {
    fn from(term: DataTerm) -> Self {
        Self {
            element: term,
            join_element: None,
            meet_elements: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn union_test1() {
        let t = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("True".to_string()),
            Vec::new(),
        ));
        let f = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("False".to_string()),
            Vec::new(),
        ));

        let l = Rc::new(RefCell::new(DataLattice::top()));
        l.as_ref().borrow_mut().union(t);
        l.as_ref().borrow_mut().union(f);

        assert!(matches!(
            DataLattice::lookup_from_top(l.clone(), &ExistsIdent::from("True")),
            Some(_)
        ));
        assert!(matches!(
            DataLattice::lookup_from_top(l.clone(), &ExistsIdent::from("False")),
            Some(_)
        ));
    }

    #[test]
    fn union_test2() {
        let t = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("True".to_string()),
            Vec::new(),
        ));
        let f = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("False".to_string()),
            Vec::new(),
        ));

        let l1 = Rc::new(RefCell::new(DataLattice::top()));
        l1.as_ref().borrow_mut().union(t);
        l1.as_ref().borrow_mut().union(f);

        let i = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("Int".to_string()),
            Vec::new(),
        ));

        let l2 = Rc::new(RefCell::new(DataLattice::top()));
        l2.as_ref().borrow_mut().union(i);

        let l3 = Rc::new(RefCell::new(DataLattice::top()));
        dbg!(&l1);
        dbg!(&l2);
        dbg!(&l1.as_ref().borrow().is_disjoint(&l2.as_ref().borrow()));
        l3.as_ref().borrow_mut().union(l1.take());
        l3.as_ref().borrow_mut().union(l2.take());
        dbg!("l3", &l3);
        assert!(false);

        assert!(matches!(
            DataLattice::lookup_from_top(l3.clone(), &ExistsIdent::from("True")),
            Some(_)
        ));
        assert!(matches!(
            DataLattice::lookup_from_top(l3.clone(), &ExistsIdent::from("False")),
            Some(_)
        ));
        assert!(matches!(
            DataLattice::lookup_from_top(l3.clone(), &ExistsIdent::from("Int")),
            Some(_)
        ));
    }

    #[test]
    fn partial_ord_test() {
        let t = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("True".to_string()),
            Vec::new(),
        ));
        let f = DataLattice::from(DataTerm::Constructor(
            DataModifier::Nominal,
            ExistsIdent("False".to_string()),
            Vec::new(),
        ));

        assert!(matches!(t.partial_cmp(&f), None));
        assert!(matches!(t.partial_cmp(&t), Some(Ordering::Equal)));

        let mut bool = DataLattice::top();
        bool.union(t);
        bool.union(f);

        assert!(matches!(
            bool.partial_cmp(&DataLattice::from(DataTerm::Constructor(
                DataModifier::Nominal,
                ExistsIdent("True".to_string()),
                Vec::new(),
            ))),
            Some(Ordering::Less)
        ));
    }
}
