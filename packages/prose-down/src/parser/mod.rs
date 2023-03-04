use anyhow::Context;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use prose_ir::Document;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct ProseParser;

pub fn parse(input: &str) -> anyhow::Result<Document> {
    ProseParser::parse(Rule::prose, input)
        .context("parse error")
        .map(document)
}

fn document(pair: Pairs<Rule>) -> prose_ir::Document {
    prose_ir::Document {
        blocks: blocks(pair),
    }
}

fn blocks(pairs: Pairs<Rule>) -> Vec<prose_ir::Block> {
    pairs
        .into_iter()
        .filter_map(|b| match b.as_rule() {
            Rule::EOI => None,
            _ => Some(block(b.into_inner().next().unwrap())),
        })
        .collect()
}

fn block(pair: Pair<Rule>) -> prose_ir::Block {
    match pair.as_rule() {
        Rule::paragraph => prose_ir::Block::Paragraph(inlines(pair.into_inner())),
        Rule::themanticBreak => prose_ir::Block::ThemanticBreak,
        Rule::emptyLine => prose_ir::Block::EmptyLine,
        _ => panic!("{}", pair),
    }
}

fn inlines(pairs: Pairs<Rule>) -> Vec<prose_ir::Inline> {
    pairs.into_iter().map(inline).collect()
}

fn inline(pair: Pair<Rule>) -> prose_ir::Inline {
    match pair.as_rule() {
        Rule::inlineExpr => prose_ir::Inline::Expr(pair.into_inner().as_str().to_string()),
        Rule::number => prose_ir::Inline::Number(pair.into_inner().as_str().to_string()),
        Rule::text => prose_ir::Inline::Text(pair.as_str().to_string()),
        Rule::EOI => prose_ir::Inline::Text("".to_string()),
        _ => panic!("{}", pair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let doc = parse("aa\nb\n---\n\nc\n{a}, ##aa##\n");
        assert!(doc.is_ok())
    }
}
