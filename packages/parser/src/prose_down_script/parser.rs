use anyhow::Context;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{
    Assign, AssignAnnotation, AssignArgs, Expr, Ident, LineComment, Literal, ParameterCondition,
    PatternExpr, Pds, TopLevel, TypeExpr,
};

#[derive(Parser)]
#[grammar = "core.pest"]
#[grammar = "prose_down_script/syntax.pest"]
pub struct PdsParser;

pub fn parse(input: &str) -> anyhow::Result<Pds> {
    let top_level = PdsParser::parse(Rule::pds, input)
        .context("")?
        .map(tokenize_top_level)
        .collect::<Vec<_>>();

    Ok(Pds(top_level))
}

fn tokenize_top_level(pair: Pair<Rule>) -> TopLevel {
    match pair.as_rule() {
        Rule::assign => TopLevel::Assign(tokenize_assign(pair)),
        Rule::assignAnnotation => TopLevel::AssignAnnotation(tokenize_assign_annotation(pair)),
        Rule::lineComment => TopLevel::LineComment(tokenize_line_comment(pair)),
        _ => panic!("{pair}"),
    }
}

fn tokenize_line_comment(pair: Pair<Rule>) -> LineComment {
    LineComment(pair.to_string())
}

fn unary(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().unwrap()
}

fn binary(pair: Pair<Rule>) -> (Pair<Rule>, Pair<Rule>) {
    let mut pairs = pair.into_inner();

    (pairs.next().unwrap(), pairs.next().unwrap())
}

fn tokenize_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr => {
            let token = unary(pair);
            match token.as_rule() {
                Rule::literal => Expr::Literal(tokenize_literal(unary(token))),
                Rule::embeddedApply => {
                    let (ident, expr) = binary(token);
                    Expr::EmbeddedApply(tokenize_ident(ident), Box::new(tokenize_expr(expr)))
                }
                _ => unreachable!("{token}"),
            }
        }
        _ => unreachable!("{pair}"),
    }
}

fn tokenize_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::textLiteral => Literal::Text(pair.into_inner().as_str().to_string()),
        Rule::charLiteral => {
            let mut p = pair.as_str().chars();
            p.next();

            Literal::Char(p.next().unwrap())
        }
        Rule::intLiteral => Literal::Int(pair.as_str().parse::<isize>().unwrap()),
        _ => panic!("{pair}"),
    }
}

fn tokenize_assign(pair: Pair<Rule>) -> Assign {
    let mut pairs = pair.into_inner();
    let ident = tokenize_ident(pairs.next().unwrap());
    let args = tokenize_assign_args(pairs.next().unwrap());
    let expr = tokenize_expr(pairs.next().unwrap());

    Assign { ident, args, expr }
}

fn tokenize_assign_args(pair: Pair<Rule>) -> AssignArgs {
    AssignArgs {
        patterns: pair.into_inner().map(tokenize_pattern).collect::<Vec<_>>(),
    }
}

fn tokenize_pattern(pair: Pair<Rule>) -> PatternExpr {
    match pair.as_rule() {
        _ => unreachable!("{pair}"),
    }
}

fn tokenize_assign_annotation(pair: Pair<Rule>) -> AssignAnnotation {
    let mut pairs = pair.into_inner();
    let ident = pairs.next().unwrap();
    let condition = pairs.next().unwrap();
    let expr = pairs.next().unwrap();

    AssignAnnotation {
        ident: tokenize_ident(ident),
        conditions: tokenize_assign_annotation_condition(condition),
        expr: tokenize_type_expr(expr),
    }
}

fn tokenize_assign_annotation_condition(pair: Pair<Rule>) -> Vec<ParameterCondition> {
    match pair.as_rule() {
        _ => unreachable!("{pair}"),
    }
}

fn tokenize_type_expr(pair: Pair<Rule>) -> TypeExpr {
    match pair.as_rule() {
        _ => unreachable!("{pair}"),
    }
}

fn tokenize_ident(pair: Pair<Rule>) -> Ident {
    match pair.as_rule() {
        Rule::ident => Ident(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}
