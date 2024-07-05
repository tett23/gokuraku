mod ast;

use anyhow::Context;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use self::ast::{Expr, PdsTokens};

#[derive(Parser)]
#[grammar = "parser/pds.pest"]
pub struct PdsParser;

pub fn parse(input: &str) -> anyhow::Result<Pds> {
    PdsParser::parse(Rule::pds, input)
        .context("")
        .map(|mut pairs| pairs.next().unwrap())
        .map(tokenize_pds)
}

use self::ast::*;
fn tokenize_pds(pair: Pair<Rule>) -> Pds {
    match pair.as_rule() {
        Rule::pds => Pds(pair
            .into_inner()
            .filter(|item| item.as_rule() != Rule::EOI)
            .map(tokenize_assign)
            .collect::<Vec<_>>()),
        _ => panic!("{pair}"),
    }
}

fn tokenize_expr(pair: Pair<Rule>) -> Expr {
    pair
    match pair.as_rule() {
        Rule::expr => match pair.into_inner().next() {
            Some(pair) => match pair.as_rule() {
                Rule::literal => Expr::Literal(tokenize_literal(pair)),
                Rule::assign => {
                    todo!()
                }
                _ => panic!("{pair}"),
            },
            None => panic!(""),
        },
        _ => panic!("{pair}"),
    }
}

fn tokenize_literal(pair: Pair<Rule>) -> Literal {
    dbg!(&pair);
    match pair.as_rule() {
        Rule::literal => {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::textLiteral => Literal::Text(pair.into_inner().as_str().to_string()),
                Rule::intLiteral => Literal::Int(pair.as_str().parse::<isize>().unwrap()),
                _ => panic!("{pair}"),
            }
        }
        _ => panic!("{pair}"),
    }
}

fn tokenize_assign(pair: Pair<Rule>) -> Assign {
    match pair.as_rule() {
        Rule::assign => {
            let vec = pair.into_inner().into_iter().collect::<Vec<_>>();
            match vec.as_slice() {
                [ident, args @ .., expr] => Assign(
                    tokenize_ident(ident.clone()),
                    args.iter()
                        .map(|item| tokenize_ident(item.clone()))
                        .collect::<Vec<_>>(),
                    Box::new(tokenize_expr(expr.clone())),
                ),
                _ => panic!(""),
            }
        }
        _ => panic!("{pair}"),
    }
}

fn tokenize_ident(pair: Pair<Rule>) -> Ident {
    match pair.as_rule() {
        Rule::ident => Ident(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let doc = parse("a b = \"1\"");
        assert!(doc.is_ok());
    }
}
