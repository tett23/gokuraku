use super::{ContextLayer, ContextStack};
use crate::ast::Node;
use anyhow::{anyhow, Result};
use handlebars::JsonValue;

#[derive(Clone)]
pub(crate) struct ContextLayerItem<T> {
    items: Vec<(String, fn(&mut ContextStack<T>, Box<dyn Node>) -> Result<T>)>,
}

impl<T> ContextLayer<T> for ContextLayerItem<T> {
    fn find(&self, name: &str) -> Option<&fn(&mut ContextStack<T>, Box<dyn Node>) -> Result<T>> {
        self.items
            .iter()
            .find(|(item, _)| item.as_str() == name)
            .map(|(_, f)| f)
    }

    fn register(&mut self, name: &str, f: fn(&mut ContextStack<T>, Box<dyn Node>) -> Result<T>) {
        self.items.push((name.to_string(), f));
    }
}

impl Default for ContextStack<String> {
    fn default() -> Self {
        Self {
            layers: vec![Box::new(ContextLayerItem::default())],
        }
    }
}

impl Default for ContextLayerItem<String> {
    fn default() -> Self {
        let mut ret = Self { items: vec![] };
        ret.register("document", document);
        ret.register("paragraph", paragraph);
        ret.register("text", text);

        ret
    }
}

fn document(ctx: &mut ContextStack<String>, node: Box<dyn Node>) -> Result<String> {
    Ok(format!(
        "{{{{#document}}}}{}{{{{/document}}}}",
        node.iter()
            .map(|child| ctx.apply(child).unwrap())
            .collect::<String>()
    ))
}

fn paragraph(ctx: &mut ContextStack<String>, node: Box<dyn Node>) -> Result<String> {
    Ok(format!(
        "{{{{#paragraph}}}}{}{{{{/paragraph}}}}",
        node.iter()
            .map(|child| ctx.apply(child).unwrap())
            .collect::<String>()
    ))
}

fn text(_ctx: &mut ContextStack<String>, node: Box<dyn Node>) -> Result<String> {
    Ok(format!(
        "{}",
        node.value()
            .unwrap_or(&JsonValue::Null)
            .as_str()
            .unwrap_or("")
    ))
}
