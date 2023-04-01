use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolTable1<T: Hash + Eq>(HashMap<T, ()>);

impl<T: Hash + Eq> Default for SymbolTable1<T> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Hash + Eq> SymbolTable1<T> {
    pub fn insert(&mut self, ident: T) -> bool {
        self.0.insert(ident, ()).is_none()
    }

    pub fn contains(&self, ident: &T) -> bool {
        self.0.contains_key(ident)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.keys()
    }
}

impl<T: Hash + Eq> IntoIterator for SymbolTable1<T> {
    type Item = <HashMap<T, ()> as IntoIterator>::Item;
    type IntoIter = <HashMap<T, ()> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
