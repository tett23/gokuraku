use anyhow::Context;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use prose_ir::Document;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct ProseParser;

pub fn parse(input: &str) -> anyhow::Result<Document> {
    let prose = ProseParser::parse(Rule::prose, input).context("parse error")?;
    let blocks = prose.into_iter().map(block).collect();

    Ok(Document { blocks })
}

fn block(pair: Pair<Rule>) -> prose_ir::Block {
    match pair.as_rule() {
        Rule::paragraph => prose_ir::Block::Paragraph(pair.into_inner().map(inline).collect()),
        Rule::themanticBreak => prose_ir::Block::ThemanticBreak,
        Rule::emptyLine => prose_ir::Block::EmptyLine,
        _ => panic!("{}", pair),
    }
}

fn inline(pair: Pair<Rule>) -> prose_ir::Inline {
    match pair.as_rule() {
        Rule::inlineExpr => prose_ir::Inline::Expr(pair.into_inner().as_str().to_string()),
        Rule::number => prose_ir::Inline::Number(pair.into_inner().as_str().to_string()),
        Rule::text => prose_ir::Inline::Text(pair.as_str().to_string()),
        _ => panic!("{}", pair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let doc = parse("aa\nb\n\n---\n\nc\n{a}, ##aa##\n");
        assert!(doc.is_ok());
    }
}
