use std::{any::Any, collections::BTreeMap};

use anyhow::{anyhow, Result};
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperDef, HelperResult, JsonValue, Output,
    RenderContext, Renderable, Template,
};
use serde_json::json;

pub struct Renderer<T> {
    pub(crate) name: String,
    pub(crate) blocks: BTreeMap<String, Box<dyn Any>>,
    pub(crate) inlines: BTreeMap<String, *const ()>,
    _marker: std::marker::PhantomData<T>,
}

// struct Block<T, U>(Box<dyn Fn() -> Box<dyn Fn(T) -> U>>);
pub struct Block<T, U>(C<T, U>);
pub struct Inline<T, U>(C<T, U>);

pub type C<U, T> = fn() -> fn(U) -> T;
// pub type D<U, T> = Fn(U) -> T;

fn a() {
    // let c: D<(), D<usize, D<usize, D<usize, String>>>> =
    //     move |_: ()| move |a: usize| move |b: usize| move |c: usize| format!("{}", a + b + c);
    // std::mem::transmute()
    // let c_addr = unsafe { std::ptr::addr_of!(c) } as *const ();
    // // let fnptr = c as *const ();
    // let fnptr: fn(i32) -> i32 = unsafe { std::mem::transmute(c_addr) };
    // assert_eq!(fnptr(40), 42);
    let mut renderer = Renderer::<String>::new("");
    // renderer.block("hoge", move || move |a: usize| format!("{}", a));
    // renderer.block("add", move || {
    //     move |a: usize| move |b: usize| format!("{}", a + b)
    // });
}

impl<T> Renderer<T> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            blocks: BTreeMap::new(),
            inlines: BTreeMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn block(&mut self, name: &str, f: Box<dyn Any>) {
        self.blocks.insert(name.to_string(), Box::new(f));
    }

    fn call(&self, name: &str, args: Vec<JsonValue>) -> Result<String> {
        let f = self
            .blocks
            .get(name)
            .ok_or_else(|| anyhow!("block not found: {}", name))?;
        dbg!(std::any::type_name_of_val(&f));

        let ret = args.into_iter().fold(
            (&f).downcast_ref::<fn() -> Box<dyn Any>>().unwrap()(),
            |f, arg| {
                dbg!(&arg);
                print_type_of(&f);
                dbg!(std::any::type_name_of_val(&f));

                if let Some(_ff) = (&f).downcast_ref::<Box<fn(usize) -> Box<dyn Any>>>() {
                    dbg!("downcasted");
                    return f.downcast::<fn(usize) -> Box<dyn Any>>().unwrap()(
                        arg.as_u64().unwrap() as usize,
                    );
                    // return f(arg.as_u64().unwrap() as usize);
                    // return f(arg.as_u64().unwrap() as usize);
                } else {
                    return f;
                }
            },
        );
        dbg!(&ret);

        let a = ret
            .downcast::<Result<String>>()
            .or_else(|_| Err(anyhow!("downcast failed")))?;

        *a
        // )?
    }

    // fn inline(&mut self, f: impl Fn() -> dyn Fn(T) -> U) {
    //     self.inlines.push(Inline(Box::new(f)));
    // }
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

impl<U, T> Block<T, U> {
    fn new(f: fn() -> fn(T) -> U) -> Self {
        // Self(Box::new(f))
        Self(f)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut renderer = Renderer::<String>::new("");
        renderer.block(
            "hoge",
            Box::new(move || Box::new(move |a: usize| format!("{}", a))),
        );
        renderer.block(
            "add",
            Box::new(move || {
                move |a: usize| move |b: usize| -> Result<String> { Ok(format!("{}", a + b)) }
            }),
        );
        let ret = renderer.call("add", vec![json!(1), json!(2)]);
        assert_eq!(ret.unwrap(), "3");
    }
}
