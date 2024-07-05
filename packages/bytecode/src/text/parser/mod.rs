use crate::{
    binary::{Expr, Instruction, Literal, Module, Signature, Symbol, TypeSymbol},
    Ir,
};
use anyhow::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser, RuleType,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "text/parser/pirt.pest"]
pub struct PdsParser;

pub fn parse(code: &str) -> Result<Ir> {
    Ok(Ir::Module(Module(
        PdsParser::parse(Rule::pirt, code)?
            .map(parse_funciton)
            .collect(),
    )))
}

fn unary<T: RuleType>(mut pairs: Pairs<T>) -> (Pair<T>,) {
    (pairs.next().unwrap(),)
}

fn binary<T: RuleType>(mut pairs: Pairs<T>) -> (Pair<T>, Pair<T>) {
    (pairs.next().unwrap(), pairs.next().unwrap())
}

fn trinary<T: RuleType>(mut pairs: Pairs<T>) -> (Pair<T>, Pair<T>, Pair<T>) {
    (
        pairs.next().unwrap(),
        pairs.next().unwrap(),
        pairs.next().unwrap(),
    )
}

fn parse_funciton(pair: Pair<Rule>) -> Ir {
    let (symbol, signature, scope) = trinary(pair.into_inner());

    Ir::FunctionSymbol(
        parse_symbol(symbol),
        parse_signature(signature),
        parse_scope(scope),
    )
}

fn parse_symbol(pair: Pair<Rule>) -> Symbol {
    Symbol(pair.as_str().to_string())
}

fn parse_signature(pair: Pair<Rule>) -> Signature {
    let (args, ret) = binary(pair.into_inner());

    Signature(parse_args(args), parse_type_symbol(ret))
}

fn parse_args(pair: Pair<Rule>) -> Vec<(Symbol, TypeSymbol)> {
    pair.into_inner().map(parse_arg).collect()
}

fn parse_arg(pair: Pair<Rule>) -> (Symbol, TypeSymbol) {
    let (symbol, ty) = binary(pair.into_inner());

    (parse_symbol(symbol), parse_type_symbol(ty))
}

fn parse_type_symbol(pair: Pair<Rule>) -> TypeSymbol {
    TypeSymbol(pair.as_str().to_string())
}

fn parse_scope(pair: Pair<Rule>) -> Expr {
    let (expr,) = unary(pair.into_inner());

    parse_expr(expr)
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::fnCall => parse_fn_call(pair),
        Rule::instCall => parse_inst_call(pair),
        Rule::literal => parse_literal(pair),
        Rule::symbolRef => parse_symbol_ref(pair),
        Rule::scope => parse_scope(pair),
        _ => unreachable!(),
    }
}

fn parse_fn_call(pair: Pair<Rule>) -> Expr {
    let (symbol_ref, args) = binary(pair.into_inner());

    Expr::FnCall(Box::new(parse_expr(symbol_ref)), parse_call(args))
}

fn parse_inst_call(pair: Pair<Rule>) -> Expr {
    let (inst, args) = binary(pair.into_inner());

    Expr::InstCall(parse_instruction(inst), parse_call(args))
}

fn parse_symbol_ref(pair: Pair<Rule>) -> Expr {
    let (symbol,) = unary(pair.into_inner());

    Expr::SymbolRef(parse_symbol(symbol))
}

fn parse_instruction(_pair: Pair<Rule>) -> Instruction {
    // TODO: parse
    Instruction::Add
}

fn parse_literal(pair: Pair<Rule>) -> Expr {
    let (literal,) = unary(pair.into_inner());

    match literal.as_rule() {
        Rule::intLiteral => Expr::Literal(Literal::Int(literal.as_str().parse().unwrap())),
        _ => unreachable!(),
    }
}

fn parse_call(pair: Pair<Rule>) -> Vec<Expr> {
    pair.into_inner().map(parse_expr).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let code = r#"
fn main(): Int {
    add(1, 2,)
}
        "#;

        let ir = parse(code);
        assert!(ir.is_ok());
    }

    #[test]
    fn test_find_main() {
        let code = r#"
fn main(): Int {
    add(1, 2,)
}
        "#;

        let ir = parse(code).unwrap();
        match ir {
            Ir::Module(module) => {
                let main = module.0.iter().find(
                    |item| matches!(item, Ir::FunctionSymbol(Symbol(main), _, _) if main == "main"),
                );

                assert!(matches!(
                    main,
                    Some(Ir::FunctionSymbol(Symbol(main), _, _) )
                if "main" == main));
            }
            _ => unreachable!(),
        }
    }
}
