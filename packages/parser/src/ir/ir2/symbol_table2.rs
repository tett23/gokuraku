use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolTable2<T: Hash + Eq, U>(HashMap<T, U>);

impl<T: Hash + Eq, U> Default for SymbolTable2<T, U> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Hash + Eq + Clone, U> SymbolTable2<T, U> {
    pub fn insert(&mut self, ident: T, value: U) -> bool {
        self.0.insert(ident, value).is_none()
    }

    pub fn contains(&self, ident: &T) -> bool {
        self.0.contains_key(ident)
    }

    pub fn find(&self, ident: &T) -> Option<&U> {
        self.0.get(ident)
    }

    pub fn find_or_default_mut(&mut self, ident: &T) -> &mut U
    where
        U: Default,
    {
        match self.contains(ident) {
            true => self.0.get_mut(ident).unwrap(),
            false => {
                self.insert(ident.clone(), U::default());
                self.0.get_mut(ident).unwrap()
            }
        }
    }

    pub fn remove(&mut self, ident: &T) -> Option<U> {
        self.0.remove(ident)
    }
}
