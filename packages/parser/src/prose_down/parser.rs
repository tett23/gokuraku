// use crate::ast::Document;
use crate::ast;
use crate::ast::*;
use anyhow::Context;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "core.pest"]
#[grammar = "prose_down_script/syntax.pest"]
#[grammar = "prose_down/syntax.pest"]
pub struct ProseParser;

pub fn parse(input: &str) -> anyhow::Result<Document> {
    let prose = ProseParser::parse(Rule::prose, input).context("parse error")?;
    let blocks = prose.into_iter().map(block).collect();

    Ok(Document { blocks })
}

fn block(pair: Pair<Rule>) -> ast::Block {
    match pair.as_rule() {
        Rule::pdsScript => ast::Block::PdsScript(pair.into_inner().as_str().to_string()),
        Rule::paragraph => ast::Block::Paragraph(pair.into_inner().map(inline).collect()),
        Rule::themanticBreak => ast::Block::ThemanticBreak,
        Rule::emptyLine => ast::Block::EmptyLine,
        _ => panic!("{}", pair),
    }
}

fn inline(pair: Pair<Rule>) -> ast::Inline {
    match pair.as_rule() {
        Rule::inlineExpr => ast::Inline::Expr(pair.into_inner().as_str().to_string()),
        Rule::number => ast::Inline::Number(pair.into_inner().as_str().to_string()),
        Rule::text => ast::Inline::Text(pair.as_str().to_string()),
        _ => panic!("{}", pair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let doc = parse("@{ a = 1 }\naa\nb\n\n---\n\nc\n{a}, ##aa##\n");
        assert!(doc.is_ok());
    }
}
