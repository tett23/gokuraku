// #![feature(tuple_trait)]
// #![feature(fn_traits)]
#![feature(core_intrinsics)]
#![feature(pointer_byte_offsets)]
#![feature(sized_type_properties)]
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

mod ast;
mod context;
mod stringify_ast;

use self::context::*;
use anyhow::{anyhow, Result};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonValue, Output, RenderContext,
    Renderable, Template,
};
use parser::ast::{Block, Document, Inline};
use serde_json::json;

pub trait TemplateContext {}

pub fn stringify(template: &str, _template_context: &dyn TemplateContext) -> Result<String> {
    let mut reg = Handlebars::new();
    // reg.set_strict_mode(true);

    reg.register_helper("paragraph", Box::new(BlockFn::Fn0(paragraph)));
    reg.register_helper("document", Box::new(BlockFn::Fn0(document)));
    reg.register_helper("inline_number", Box::new(InlineFn::Fn2(inline_number)));

    let ret = reg.render_template(template, &json!({"bar": 1, "a": 2}))?;

    Ok(ret)
}

pub fn stringify_ast(
    document: &Document,
    _template_context: &dyn TemplateContext,
) -> Result<String> {
    let mut reg = Handlebars::new();
    // reg.set_strict_mode(true);

    // reg.register_helper("paragraph", Box::new(BlockFn::Fn0(paragraph)));
    // reg.register_helper("document", Box::new(BlockFn::Fn0(document)));
    // reg.register_helper("inline_number", Box::new(InlineFn::Fn2(inline_number)));

    // let ret = reg.render_template(document, &json!({"bar": 1, "a": 2}))?;
    let ret = "TODO".to_string();

    Ok(ret)
}

#[derive(Clone, Copy)]
pub enum InlineFn {
    Fn0(fn() -> Result<String>),
    Fn1(fn(&JsonValue) -> Result<String>),
    Fn2(fn(&JsonValue, &JsonValue) -> Result<String>),
    Fn3(fn(&JsonValue, &JsonValue, &JsonValue) -> Result<String>),
    Fn4(fn(&JsonValue, &JsonValue, &JsonValue, &JsonValue) -> Result<String>),
    Fn5(fn(&JsonValue, &JsonValue, &JsonValue, &JsonValue, &JsonValue) -> Result<String>),
}

#[derive(Clone, Copy)]
pub enum BlockFn {
    Fn0(fn(&mut PdsContext, &Template) -> Result<String>),
    Fn1(fn(&mut PdsContext, &Template, &JsonValue) -> Result<String>),
    Fn2(fn(&mut PdsContext, &Template, &JsonValue, &JsonValue) -> Result<String>),
    Fn3(fn(&mut PdsContext, &Template, &JsonValue, &JsonValue, &JsonValue) -> Result<String>),
    Fn4(
        fn(
            &mut PdsContext,
            &Template,
            &JsonValue,
            &JsonValue,
            &JsonValue,
            &JsonValue,
        ) -> Result<String>,
    ),
    Fn5(
        fn(
            &mut PdsContext,
            &Template,
            &JsonValue,
            &JsonValue,
            &JsonValue,
            &JsonValue,
            &JsonValue,
        ) -> Result<String>,
    ),
}

impl From<&fn() -> Result<String>> for InlineFn {
    fn from(value: &fn() -> Result<String>) -> Self {
        InlineFn::Fn0(*value)
    }
}

pub struct PdsContext<'reg: 'rc, 'rc, 'a> {
    template: Option<&'reg Template>,
    handlebars: &'reg Handlebars<'reg>,
    ctx: &'rc Context,
    rc: &'a mut RenderContext<'reg, 'rc>,
    out: &'a mut dyn Output,
}

trait Callable {
    fn call(
        &self,
        ctx: &mut PdsContext,
        template: Option<&Template>,
        args: Vec<JsonValue>,
    ) -> Result<String>;
}

impl Callable for InlineFn {
    fn call(
        &self,
        _ctx: &mut PdsContext,
        _template: Option<&Template>,
        args: Vec<JsonValue>,
    ) -> Result<String> {
        match (self, &args.as_slice()) {
            (InlineFn::Fn0(f), []) => f(),
            (InlineFn::Fn1(f), [arg1]) => f(arg1),
            (InlineFn::Fn2(f), [arg1, arg2]) => f(arg1, arg2),
            (InlineFn::Fn3(f), [arg1, arg2, arg3]) => f(arg1, arg2, arg3),
            (InlineFn::Fn4(f), [arg1, arg2, arg3, arg4]) => f(arg1, arg2, arg3, arg4),
            (InlineFn::Fn5(f), [arg1, arg2, arg3, arg4, arg5]) => f(arg1, arg2, arg3, arg4, arg5),
            _ => unreachable!(),
        }
    }
}

impl Callable for BlockFn {
    fn call(
        &self,
        ctx: &mut PdsContext,
        template: Option<&Template>,
        args: Vec<JsonValue>,
    ) -> Result<String> {
        match (self, &args.as_slice()) {
            (BlockFn::Fn0(f), []) => f(ctx, template.unwrap()),
            (BlockFn::Fn1(f), [arg1]) => f(ctx, template.unwrap(), arg1),
            (BlockFn::Fn2(f), [arg1, arg2]) => f(ctx, template.unwrap(), arg1, arg2),
            (BlockFn::Fn3(f), [arg1, arg2, arg3]) => f(ctx, template.unwrap(), arg1, arg2, arg3),
            (BlockFn::Fn4(f), [arg1, arg2, arg3, arg4]) => {
                f(ctx, template.unwrap(), arg1, arg2, arg3, arg4)
            }
            (BlockFn::Fn5(f), [arg1, arg2, arg3, arg4, arg5]) => {
                f(ctx, template.unwrap(), arg1, arg2, arg3, arg4, arg5)
            }
            _ => unreachable!(),
        }
    }
}

impl HelperDef for InlineFn {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        handlebars: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let args = h
            .params()
            .iter()
            .map(|item| item.value().clone())
            .collect::<Vec<_>>();

        let new_tmpl = Callable::call(
            self,
            &mut PdsContext {
                template: h.template(),
                handlebars,
                ctx,
                rc,
                out,
            },
            None,
            args,
        );

        let new_tmpl = new_tmpl.map(|tmpl| {
            handlebars
                .render_template(&tmpl, &json!({}))
                .unwrap_or("to_string".to_string())
        });
        out.write(&new_tmpl.unwrap_or("".to_string()))?;
        Ok(())
    }
}

impl HelperDef for BlockFn {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        handlebars: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let args = h
            .params()
            .iter()
            .map(|item| item.value().clone())
            .collect::<Vec<_>>();

        let new_tmpl = Callable::call(
            self,
            &mut PdsContext {
                template: h.template(),
                handlebars,
                ctx,
                rc,
                out,
            },
            h.template(),
            args,
        );
        let new_tmpl = new_tmpl.map(|tmpl| {
            handlebars
                .render_template(&tmpl, &json!({}))
                .unwrap_or("to_string".to_string())
        });
        out.write(&new_tmpl.unwrap_or("".to_string()))?;

        Ok(())
    }
}

fn paragraph(ctx: &mut PdsContext, _template: &Template) -> Result<String> {
    let item = ctx
        .template
        .unwrap()
        .renders(ctx.handlebars, ctx.ctx, ctx.rc)
        .unwrap();
    Ok(format!("<p>{item}</p>"))
}

fn document(ctx: &mut PdsContext, _template: &Template) -> Result<String> {
    let item = ctx
        .template
        .unwrap()
        .renders(ctx.handlebars, ctx.ctx, ctx.rc)
        .unwrap();
    Ok(format!("<doc>{item}</doc>"))
}

fn inline_number(p: &JsonValue, _formatter: &JsonValue) -> Result<String> {
    p.as_i64()
        .map(|p| format!("{}", p))
        .ok_or(anyhow!("not a number"))
}

#[derive(Debug, Clone, Default)]
struct DefaultContext {}
impl TemplateContext for DefaultContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = stringify("{{#document}}{{#paragraph}}{{bbbbb}}{{a}}{{foo}}bb{{/paragraph}}{{foo}}\n{{baz}}{{inline_number 1000 a}}{{/document}}", &DefaultContext::default());
        dbg!(result);
        assert!(false)
    }
}
