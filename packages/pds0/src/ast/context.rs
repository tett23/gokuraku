use crate::ast::Node;
use anyhow::Result;

#[derive(Default)]
pub struct ContextStack<'a, 'b, T> {
    layers: Vec<Box<dyn ContextLayer<'a, 'b, T>>>,
}

impl<'a, 'b, T> ContextStack<'a, 'b, T> {
    pub fn lookup(
        &self,
        name: &str,
    ) -> Option<&fn(&mut ContextStack<'a, 'b, T>, &'b dyn Node) -> Result<T>> {
        self.layers
            .iter()
            .find(|layer| layer.find(name).is_some())
            .map(|layer| {
                layer
                    .find(name)
                    .expect("layer.find(name) is_some but layer.find(name) is None")
            })
    }

    pub fn push(&mut self, layer: Box<dyn ContextLayer<'a, 'b, T>>) {
        self.layers.push(layer);
    }

    pub fn pop(&mut self) {
        self.layers.pop();
    }
}

pub trait ContextLayer<'a, 'b, T> {
    fn find(
        &self,
        name: &str,
    ) -> Option<&for<'f, 'g> fn(&'a mut ContextStack<'f, 'g, T>, &'g dyn Node) -> Result<T>>;
    fn register(
        &mut self,
        name: &str,
        f: fn(&'a mut dyn ContextLayer<T>, &'b dyn Node) -> Result<T>,
    );
}

#[derive(Clone, Default)]
pub(crate) struct ContextLayerItem<'a, 'b, T> {
    items: Vec<(
        String,
        fn(&'a mut ContextStack<T>, &'b dyn Node) -> Result<T>,
    )>,
}

impl<'a, 'b, T> ContextLayer<'a, 'b, T> for ContextLayerItem<'a, 'b, T> {
    fn find<'s>(
        &'s self,
        name: &str,
    ) -> Option<&'s for<'f, 'g> fn(&'a mut ContextStack<'f, 'g, T>, &'g dyn Node) -> Result<T>>
    {
        let a = self
            .items
            .iter()
            .find(|(item, _)| item.as_str() == name)
            .map(|(_, f)| f);
        a
    }

    fn register(
        &mut self,
        name: &str,
        f: fn(&'a mut dyn ContextLayer<T>, &'b dyn Node) -> Result<T>,
    ) {
    }
}

// #[derive(Clone, Default)]
// pub(crate) struct DefaultContext<'a, 'b> {
//     items: Vec<(
//         String,
//         fn(&'a mut ContextStack<String>, &'b dyn Node) -> Result<String>,
//     )>,
// }
