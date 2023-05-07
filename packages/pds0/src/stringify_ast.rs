use crate::ast::{ContextStack, Node, Pds0Ast, Pds0Literal, Pds0Node, Pds0Parent};
use anyhow::{anyhow, Result};
use handlebars::JsonValue;
use parser::ast::{Block, Document, Inline};
use std::collections::BTreeMap;

pub fn stringify_ast<'a, 'b>(
    ctx: &mut ContextStack<'a, 'b, String>,
    document: Document,
) -> Result<String> {
    let mut reg = handlebars::Handlebars::new();
    // reg.set_strict_mode(true);

    // reg.register_helper("paragraph", Box::new(BlockFn::Fn0(paragraph)));
    // reg.register_helper("document", Box::new(BlockFn::Fn0(document)));
    // reg.register_helper("inline_number", Box::new(InlineFn::Fn2(inline_number)));

    // let ret = reg.render_template(document, &json!({"bar": 1, "a": 2}))?;
    let ast = Box::new(Pds0Ast::from(document)) as Box<dyn Node>;
    let ret = block(ctx, ast.as_ref())?;

    Ok(ret)
}

fn block<'a, 'b>(ctx: &'a mut ContextStack<'a, 'b, String>, node: &'b dyn Node) -> Result<String> {
    let f = ctx
        .lookup(node.name())
        .ok_or(anyhow!("block not found: {}", node.name()))?;
    // .map(|f| f(ctx, node).unwrap())

    let a = f(ctx, node);

    a
}

// fn inline(ctx: &mut ContextStack<String>, node: &dyn Node) -> Result<String> {
//     ctx.lookup(node.name())
//         .ok_or(anyhow!("inline not found: {}", node.name()))?(ctx, node)
// }

macro_rules! pds_parent {
    ($name:ident, {}, []) => {
        Pds0Parent {
            name: stringify!($name).to_string(),
            data: std::collections::BTreeMap::new(),
            children: vec![],
        }
    };
    () => {};
}

// impl From<Document> for Pds0Ast {
//     fn from(document: Document) -> Self {
//         Pds0Ast::Parent(pds_parent!(document, {}, []))
//     }
// }

impl From<Document> for Pds0Ast {
    fn from(value: Document) -> Self {
        Pds0Ast::Parent(Pds0Parent {
            name: "document".to_string(),
            data: BTreeMap::new(),
            children: value.0.into_iter().map(Pds0Ast::from).collect(),
        })
    }
}

impl From<Block> for Pds0Ast {
    fn from(value: Block) -> Self {
        match value {
            Block::EmptyLine => Pds0Ast::Node(Pds0Node {
                name: "empty_line".to_string(),
                data: BTreeMap::new(),
            }),
            Block::ThemanticBreak => Pds0Ast::Node(Pds0Node {
                name: "themantic_break".to_string(),
                data: BTreeMap::new(),
            }),
            Block::PdsScript(value) => Pds0Ast::Node(Pds0Node {
                name: "pds_script".to_string(),
                data: {
                    let mut map = BTreeMap::new();
                    map.insert("value".to_string(), JsonValue::String(value));
                    map
                },
            }),
            Block::Paragraph(value) => Pds0Ast::Parent(Pds0Parent {
                name: "paragraph".to_string(),
                data: BTreeMap::new(),
                children: value.into_iter().map(Pds0Ast::from).collect(),
            }),
        }
    }
}

impl From<Inline> for Pds0Ast {
    fn from(value: Inline) -> Self {
        match value {
            Inline::Text(value) => Pds0Ast::Literal(Pds0Literal {
                name: "text".to_string(),
                data: BTreeMap::new(),
                value: JsonValue::String(value),
            }),
            Inline::Number(value) => Pds0Ast::Parent(Pds0Parent {
                name: "number".to_string(),
                data: BTreeMap::new(),
                children: vec![Pds0Ast::Literal(Pds0Literal {
                    name: "number".to_string(),
                    data: BTreeMap::new(),
                    value: JsonValue::String(value),
                })],
            }),
            Inline::Expr(value) => Pds0Ast::Literal(Pds0Literal {
                name: "expr".to_string(),
                data: {
                    let mut map = BTreeMap::new();
                    map.insert("value".to_string(), JsonValue::String(value.clone()));
                    map
                },
                value: JsonValue::String(value),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ContextLayer;
    use parser::prose_down_parse;

    #[test]
    fn test() {
        let mut ctx = ContextStack::default();

        let ast = prose_down_parse("Hello, world!\n").unwrap();
        let result = stringify_ast(&mut ctx, &ast);
        dbg!(result);
        assert!(false)
    }
}
