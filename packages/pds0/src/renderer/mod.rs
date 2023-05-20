use anyhow::{anyhow, Result};
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperDef, HelperResult, JsonValue, Output,
    RenderContext, Renderable, Template,
};
use serde_json::json;

pub struct Renderer<T, U> {
    pub(crate) name: String,
    pub(crate) blocks: Vec<Box<Block<T, U>>>,
    pub(crate) inlines: Vec<Inline<T, U>>,
}

// struct Block<T, U>(Box<dyn Fn() -> Box<dyn Fn(T) -> U>>);
struct Block<T, U>(dyn (Fn() -> dyn Fn(T) -> U) + Send + Sync);
struct Inline<T, U>(Box<dyn Fn() -> Box<dyn Fn(T) -> U>>);

fn a() {
    let renderer = Renderer::new("");
}

impl<T, U> Renderer<T, U> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            blocks: Vec::new(),
            inlines: Vec::new(),
        }
    }

    fn block(&mut self, f: impl (Fn() -> dyn Fn(T) -> U) + Send + Sync) {
        self.blocks.push(Box::new(Block(f)));
    }

    fn inline(&mut self, f: impl Fn() -> dyn Fn(T) -> U) {
        self.inlines.push(Inline(Box::new(f)));
    }
}

// fn to_helper(
//     f: Box<dyn Fn() -> Box<dyn Fn() -> Result<String>>>,
// ) -> Box<dyn HelperDef + Send + Sync> {
//     Box::new(
//         move |h: &Helper<'reg, 'rc>,
//               handlebars: &'reg Handlebars<'reg>,
//               ctx: &'rc Context,
//               rc: &mut RenderContext<'reg, 'rc>,
//               out: &mut dyn Output| {
//             Box::new(f);
//             Ok(())
//         },
//     )
// }
