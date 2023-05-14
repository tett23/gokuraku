use super::ContextLayer;
use crate::ast::Node;
use anyhow::{anyhow, Result};

pub struct ContextStack<T> {
    pub(crate) layers: Vec<Box<dyn ContextLayer<T>>>,
}

impl<T> ContextStack<T> {
    pub fn lookup(
        &self,
        name: &str,
    ) -> Option<&fn(&mut ContextStack<T>, Box<dyn Node>) -> Result<T>> {
        self.layers
            .iter()
            .rfind(|layer| layer.find(name).is_some())
            .map(|layer| {
                layer
                    .find(name)
                    .expect("layer.find(name) is_some but layer.find(name) is None")
            })
    }

    pub fn push(&mut self, layer: Box<dyn ContextLayer<T>>) {
        self.layers.push(layer);
    }

    pub fn pop(&mut self) {
        self.layers.pop();
    }

    pub fn apply(&mut self, node: Box<dyn Node>) -> Result<T> {
        let name = node.name();
        let f = self
            .lookup(name)
            .ok_or_else(|| anyhow!("No function found for {}", name))?;
        f(self, node)
    }
}
