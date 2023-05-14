use super::ContextStack;
use crate::ast::Node;
use anyhow::Result;

pub trait ContextLayer<T> {
    fn find(&self, name: &str) -> Option<&fn(&mut ContextStack<T>, Box<dyn Node>) -> Result<T>>;
    fn register(&mut self, name: &str, f: fn(&mut ContextStack<T>, Box<dyn Node>) -> Result<T>);
}
