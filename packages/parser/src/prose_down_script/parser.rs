use anyhow::Context;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{
    Abstruction, Apply, ApplyEff, ApplyInst, Assign, AssignArgs, AssignDef, CoroutineType,
    DataAssign, DataConstructor, DataExpr, DataModifier, DataValue, EtaEnv, EtaEnvs, ExistsIdent,
    Expr, ForallIdent, HandlerAssign, HandlerDef, HandlerIdent, HandlerTypeDefExpr, Ident,
    ImplTrait, InstDef, InstIdent, LineComment, Literal, Module, PatternExpr, Statement,
    TraitConstraint, TraitDef, TraitIdent, TypeAbstructionExpr, TypeExpr, TypeIdent, TypeLiteral,
};

fn unary(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().unwrap()
}

fn binary(pair: Pair<Rule>) -> (Pair<Rule>, Pair<Rule>) {
    let mut pairs = pair.into_inner();

    (pairs.next().unwrap(), pairs.next().unwrap())
}

fn triple(pair: Pair<Rule>) -> (Pair<Rule>, Pair<Rule>, Pair<Rule>) {
    let mut pairs = pair.into_inner();

    (
        pairs.next().unwrap(),
        pairs.next().unwrap(),
        pairs.next().unwrap(),
    )
}

fn quadruple(pair: Pair<Rule>) -> (Pair<Rule>, Pair<Rule>, Pair<Rule>, Pair<Rule>) {
    let mut pairs = pair.into_inner();

    (
        pairs.next().unwrap(),
        pairs.next().unwrap(),
        pairs.next().unwrap(),
        pairs.next().unwrap(),
    )
}

fn unary_or_binary(pair: Pair<Rule>) -> (Pair<Rule>, Option<Pair<Rule>>) {
    let mut pairs = pair.into_inner();

    (pairs.next().unwrap(), pairs.next())
}

fn unary_or_none(pair: Pair<Rule>) -> Option<Pair<Rule>> {
    let mut pairs = pair.into_inner();

    pairs.next()
}

#[derive(Parser)]
// #[grammar = "core.pest"]
#[grammar = "prose_down_script/syntax.pest"]
pub struct PdsParser;

pub fn parse(input: &str) -> anyhow::Result<Module> {
    let stmts = PdsParser::parse(Rule::root, input)
        .context("parse error")?
        .map(parse_stmt)
        .collect::<Vec<_>>();

    Ok(Module { statements: stmts })
}

pub fn parse_module(pair: Pair<Rule>) -> Module {
    let module = pair.into_inner().map(parse_stmt).collect::<Vec<_>>();

    Module { statements: module }
}

fn parse_stmt(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::assignTypeDef => Statement::AssignDef(parse_assign_def(pair)),
        Rule::assign => Statement::Assign(parse_assign(pair)),
        Rule::traitDef => Statement::TraitDef(parse_trait_def(pair)),
        Rule::implAssign => Statement::ImplTrait(parse_impl_trait(pair)),
        Rule::instTypeDef => Statement::InstDef(parse_inst_def(pair)),
        Rule::handlerTypeDef => Statement::HandlerDef(parse_handler_def(pair)),
        Rule::handlerAssign => Statement::HandlerAssign(parse_handler_assign(pair)),
        Rule::dataAssign => Statement::DataAssign(parse_data_assign(pair)),
        Rule::lineComment => Statement::LineComment(parse_line_comment(pair)),
        Rule::stmt => parse_stmt(unary(pair)),
        _ => panic!("{pair}"),
    }
}

fn parse_impl_trait(pair: Pair<Rule>) -> ImplTrait {
    let (constraints, ident, args, where_clause) = quadruple(pair);

    ImplTrait {
        constraints: constraints
            .into_inner()
            .map(parse_trait_constraint)
            .collect::<Vec<_>>(),
        ident: parse_type_ident(ident),
        args: args.into_inner().map(parse_type_ident).collect::<Vec<_>>(),
        where_clause: parse_where_clause(where_clause),
    }
}

fn parse_data_assign(pair: Pair<Rule>) -> DataAssign {
    let (modifier, constraints, ident, args, expr) = {
        let mut pairs = pair.into_inner();

        (
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
        )
    };

    DataAssign {
        modifier: parse_data_modifier(modifier),
        constraints: constraints
            .into_inner()
            .map(parse_trait_constraint)
            .collect::<Vec<_>>(),
        ident: parse_type_ident(ident),
        args: args.into_inner().map(parse_type_ident).collect::<Vec<_>>(),
        expr: parse_data_expr(expr),
    }
}

fn parse_data_modifier(pair: Pair<Rule>) -> Option<DataModifier> {
    match pair.as_rule() {
        Rule::dataModifierNominal => Some(DataModifier::Nominal),
        Rule::dataModifierStructual => Some(DataModifier::Structual),
        Rule::dataModifier => unary_or_none(pair).and_then(parse_data_modifier),
        _ => panic!("{pair}"),
    }
}

fn parse_data_expr(pair: Pair<Rule>) -> DataExpr {
    match pair.as_rule() {
        Rule::dataExprOr => parse_data_expr_or(pair),
        Rule::dataValue => parse_data_value(unary(pair)),
        Rule::dataValueGroup => parse_data_expr(unary(pair)),
        Rule::dataExpr => parse_data_expr(unary(pair)),
        _ => panic!("{pair}"),
    }
}

fn parse_data_expr_or(pair: Pair<Rule>) -> DataExpr {
    let (lhs, rhs) = binary(pair);

    DataExpr::Or(
        Box::new(parse_data_expr(lhs)),
        Box::new(parse_data_expr(rhs)),
    )
}

fn parse_data_value(pair: Pair<Rule>) -> DataExpr {
    match pair.as_rule() {
        Rule::dataValueConstructor => {
            DataExpr::Value(DataValue::Constructor(parse_type_expr_constructor(pair)))
        }
        Rule::dataValueUnit => DataExpr::Value(DataValue::Unit),
        Rule::dataValueGroup => parse_data_expr(unary(pair)),
        Rule::dataValue => parse_data_value(unary(pair)),
        _ => panic!("{pair}"),
    }
}

fn parse_handler_def(pair: Pair<Rule>) -> HandlerDef {
    let (ident, expr) = binary(pair);

    HandlerDef {
        ident: parse_handler_ident(ident),
        expr: parse_handler_type_def_expr(expr),
    }
}

fn parse_handler_type_def_expr(pair: Pair<Rule>) -> HandlerTypeDefExpr {
    let (constraints, eta_envs, expr) = triple(pair);

    HandlerTypeDefExpr {
        trait_constraints: constraints
            .into_inner()
            .map(parse_trait_constraint)
            .collect::<Vec<_>>(),
        eta_envs: parse_eta_envs(eta_envs),
        expr: parse_type_abstruction_expr(expr),
    }
}

fn parse_inst_def(pair: Pair<Rule>) -> InstDef {
    let (ident, expr) = binary(pair);

    InstDef {
        ident: parse_inst_ident(ident),
        expr: parse_type_expr(expr),
    }
}

fn parse_trait_def(pair: Pair<Rule>) -> TraitDef {
    let (constraints, ident, args, where_clause) = quadruple(pair);

    TraitDef {
        trait_constraints: constraints
            .into_inner()
            .map(parse_trait_constraint)
            .collect::<Vec<_>>(),
        constructor: DataConstructor {
            modifier: None,
            ident: parse_type_ident(ident),
            args: args
                .into_inner()
                .map(parse_type_literal)
                .collect::<Vec<_>>(),
        },
        where_clause: parse_where_clause(where_clause),
    }
}

fn parse_where_clause(pair: Pair<Rule>) -> Module {
    let pair = pair.into_inner().next();

    match pair {
        Some(pair) => parse_module(pair),
        None => Module { statements: vec![] },
    }
}

fn parse_eta_envs(pair: Pair<Rule>) -> EtaEnvs {
    EtaEnvs(pair.into_inner().map(parse_eta_env).collect::<Vec<_>>())
}

fn parse_eta_env(pair: Pair<Rule>) -> EtaEnv {
    let (ident, handler_expr) = binary(pair);

    EtaEnv {
        ident: parse_handler_ident(ident),
        expr: parse_handler_type_expr(handler_expr),
    }
}

fn parse_handler_type_expr(pair: Pair<Rule>) -> CoroutineType {
    let (resume, ret) = binary(pair);

    CoroutineType {
        resume: parse_type_abstruction_expr(unary(resume)),
        ret: parse_type_abstruction_expr(unary(ret)),
    }
}

fn parse_trait_constraint(pair: Pair<Rule>) -> TraitConstraint {
    let (ident, arg) = binary(pair);

    TraitConstraint {
        ident: parse_trait_ident(ident),
        arg: parse_forall_ident(arg),
    }
}

fn parse_line_comment(pair: Pair<Rule>) -> LineComment {
    LineComment(pair.to_string())
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::literal => Expr::Literal(parse_literal(unary(pair))),
        Rule::apply => parse_apply(pair),
        Rule::instApply => parse_inst_apply(pair),
        Rule::effApply => parse_eff_apply(pair),
        Rule::variable => Expr::Ident(parse_ident(unary(pair))),
        Rule::expr => parse_expr(unary(pair)),
        Rule::infixIdentSeparator => Expr::Ident(parse_ident(pair)),
        Rule::infixOperator => Expr::Ident(parse_ident(pair)),
        _ => unreachable!("{pair}"),
    }
}

fn parse_apply(pair: Pair<Rule>) -> Expr {
    let pair = unary(pair);
    match pair.as_rule() {
        Rule::applyPrefix => parse_apply_prefix(unary(pair)),
        Rule::applyInfix => parse_apply_infix(pair),
        Rule::apply => parse_apply(unary(pair)),
        _ => unreachable!("{pair}"),
    }
}

fn parse_apply_prefix(pair: Pair<Rule>) -> Expr {
    let (ident, expr) = unary_or_binary(pair);

    match expr {
        Some(expr) => Expr::Apply(Apply {
            abstruction: Abstruction {
                arg: None,
                expr: Box::new(parse_expr(ident)),
            },
            expr: Box::new(parse_expr(expr)),
        }),
        None => Expr::Ident(parse_ident(ident)),
    }
}

fn parse_apply_infix(pair: Pair<Rule>) -> Expr {
    let (lhs, ident, rhs) = triple(pair);

    Expr::Apply(Apply {
        abstruction: Abstruction {
            arg: Some(Box::new(parse_expr(lhs))),
            expr: Box::new(parse_expr(ident)),
        },
        expr: Box::new(parse_expr(rhs)),
    })
}

fn parse_inst_apply(pair: Pair<Rule>) -> Expr {
    let (ident, expr) = unary_or_binary(pair);

    match expr {
        Some(expr) => Expr::ApplyInst(ApplyInst {
            ident: parse_inst_ident(ident),
            expr: Box::new(parse_expr(expr)),
        }),
        None => Expr::InstIdent(parse_inst_ident(ident)),
    }
}

fn parse_eff_apply(pair: Pair<Rule>) -> Expr {
    let (ident, expr) = unary_or_binary(pair);

    match expr {
        Some(expr) => Expr::ApplyEff(ApplyEff {
            ident: parse_handler_ident(ident),
            expr: Box::new(parse_expr(expr)),
        }),
        None => Expr::HandlerIdent(parse_handler_ident(ident)),
    }
}

fn parse_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::textLiteral => Literal::Text(pair.into_inner().as_str().to_string()),
        Rule::charLiteral => {
            let mut p = pair.as_str().chars();
            p.next();

            Literal::Char(p.next().unwrap())
        }
        Rule::intLiteral => Literal::Int(pair.as_str().parse::<isize>().unwrap()),
        Rule::unitLiteral => Literal::Unit,
        Rule::arrayLiteral => Literal::Array(pair.into_inner().map(parse_expr).collect::<Vec<_>>()),
        Rule::tupleLiteral => parse_tuple(unary(pair)),
        Rule::literal => parse_literal(unary(pair)),
        _ => panic!("{pair}"),
    }
}

fn parse_tuple(pair: Pair<Rule>) -> Literal {
    let tuple = pair.into_inner().map(parse_expr).collect::<Vec<_>>();

    Literal::Tuple(tuple.len(), tuple)
}

fn parse_assign(pair: Pair<Rule>) -> Assign {
    let (ident, args, expr, where_clause) = quadruple(pair);

    Assign {
        ident: parse_ident(ident),
        args: parse_assign_args(args),
        expr: parse_expr(expr),
        where_clause: parse_where_clause(where_clause),
    }
}

fn parse_handler_assign(pair: Pair<Rule>) -> HandlerAssign {
    let (ident, args, expr, where_clause) = quadruple(pair);

    HandlerAssign {
        ident: parse_handler_ident(ident),
        args: parse_assign_args(args),
        expr: parse_expr(expr),
        where_clause: parse_where_clause(where_clause),
    }
}

fn parse_assign_args(pair: Pair<Rule>) -> AssignArgs {
    AssignArgs {
        patterns: pair.into_inner().map(parse_pattern).collect::<Vec<_>>(),
    }
}

fn parse_pattern(pair: Pair<Rule>) -> PatternExpr {
    match pair.as_rule() {
        Rule::patternOr => {
            let (lhs, rhs) = binary(pair);
            PatternExpr::Or(
                Box::new(parse_pattern(lhs)),
                Box::new(parse_pattern(unary(rhs))),
            )
        }
        Rule::patternLiteral => PatternExpr::Literal(parse_literal(unary(pair))),
        Rule::patternIdent => PatternExpr::Bind(parse_ident(unary(pair))),
        Rule::patternListHead => {
            todo!()
        }
        Rule::patternAny => PatternExpr::Any,
        Rule::patternConstructor => {
            PatternExpr::Constructor(parse_type_expr_constructor(unary(pair)))
        }
        Rule::pattern => parse_pattern(unary(pair)),
        _ => unreachable!("{pair}"),
    }
}

fn parse_assign_def(pair: Pair<Rule>) -> AssignDef {
    let (ident, type_expr) = binary(pair);

    AssignDef {
        ident: parse_ident(ident),
        expr: parse_type_expr(type_expr),
    }
}

fn parse_type_expr(pair: Pair<Rule>) -> TypeExpr {
    let (constraints, eta_envs, expr) = triple(pair);

    TypeExpr {
        trait_constraints: constraints
            .into_inner()
            .map(parse_trait_constraint)
            .collect::<Vec<_>>(),
        eta_envs: parse_eta_envs(eta_envs),
        expr: parse_type_abstruction_expr(unary(expr)),
    }
}

fn parse_type_abstruction_expr(pair: Pair<Rule>) -> TypeAbstructionExpr {
    match pair.as_rule() {
        Rule::typeExprArrow => {
            let (lhs, rhs) = binary(pair);
            TypeAbstructionExpr::Arrow(
                Box::new(parse_type_abstruction_expr(lhs)),
                Box::new(parse_type_abstruction_expr(unary(rhs))),
            )
        }
        Rule::typeExprConstructor => TypeAbstructionExpr::Literal(TypeLiteral::Constructor(
            parse_type_expr_constructor(pair),
        )),
        Rule::typeExprGroup => parse_type_abstruction_expr(unary(pair)),
        Rule::typeExprLiteral => TypeAbstructionExpr::Literal(parse_type_literal(unary(pair))),
        Rule::abstructionTypeExpr => parse_type_abstruction_expr(unary(pair)),
        Rule::typeExprCoroutine => TypeAbstructionExpr::Literal(parse_type_literal(pair)),
        _ => unreachable!("{pair}"),
    }
}

fn parse_type_literal(pair: Pair<Rule>) -> TypeLiteral {
    match pair.as_rule() {
        Rule::typeExprUnit => TypeLiteral::Tuple(0, Vec::new()),
        Rule::typeExprTuple => {
            let items = pair
                .into_inner()
                .map(parse_type_abstruction_expr)
                .collect::<Vec<_>>();
            TypeLiteral::Tuple(items.len(), items)
        }
        Rule::typeExprArray => {
            TypeLiteral::Array(Box::new(parse_type_abstruction_expr(unary(unary(pair)))))
        }
        Rule::typeExprDivergent => TypeLiteral::Bottom,
        Rule::typeExprConstructor => {
            TypeLiteral::Constructor(parse_type_expr_constructor(unary(pair)))
        }
        Rule::dataValueConstructor => TypeLiteral::Constructor(parse_type_expr_constructor(pair)),
        Rule::typeIdent => TypeLiteral::Constructor(parse_type_expr_constructor(pair)),
        Rule::typeExprCoroutine => {
            TypeLiteral::Coroutine(Box::new(parse_type_expr_coroutine(pair)))
        }
        Rule::typeExprTop => TypeLiteral::Top,
        Rule::dataValue => parse_type_literal(unary(pair)),
        _ => unreachable!("{pair}"),
    }
}

fn parse_type_expr_constructor(pair: Pair<Rule>) -> DataConstructor {
    match pair.as_rule() {
        Rule::dataValue => parse_type_expr_constructor(unary(pair)),
        Rule::dataValueConstructor => {
            let (modifier, ident, args) = triple(pair);

            DataConstructor {
                modifier: parse_data_modifier(modifier),
                ident: parse_type_ident(ident),
                args: args
                    .into_inner()
                    .map(parse_type_literal)
                    .collect::<Vec<_>>(),
            }
        }
        Rule::typeIdent => DataConstructor {
            modifier: None,
            ident: parse_type_ident(pair),
            args: Vec::new(),
        },
        Rule::typeExprConstructor => parse_type_expr_constructor(unary(pair)),
        _ => unreachable!("{pair}"),
    }
}

fn parse_type_expr_coroutine(pair: Pair<Rule>) -> CoroutineType {
    let (resume, ret) = binary(pair);

    CoroutineType {
        resume: parse_type_abstruction_expr(resume),
        ret: parse_type_abstruction_expr(ret),
    }
}

fn parse_ident(pair: Pair<Rule>) -> Ident {
    match pair.as_rule() {
        Rule::ident => Ident(pair.as_str().to_string()),
        Rule::varIdent => Ident(pair.as_str().to_string()),
        Rule::infixOperator => parse_ident(unary(pair)),
        Rule::operator => Ident(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}

fn parse_type_ident(pair: Pair<Rule>) -> TypeIdent {
    match pair.as_rule() {
        Rule::forallIdent => TypeIdent::ForallIdent(ForallIdent(pair.as_str().to_string())),
        Rule::existsIdent => TypeIdent::ExistsIdent(ExistsIdent(pair.as_str().to_string())),
        Rule::typeIdent => parse_type_ident(unary(pair)),
        _ => panic!("{pair}"),
    }
}

fn parse_handler_ident(pair: Pair<Rule>) -> HandlerIdent {
    match pair.as_rule() {
        Rule::handlerIdent => HandlerIdent(pair.as_str().to_string()),
        Rule::etaHandlerIdent => HandlerIdent(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}

fn parse_inst_ident(pair: Pair<Rule>) -> InstIdent {
    match pair.as_rule() {
        Rule::instIdent => InstIdent(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}

fn parse_trait_ident(pair: Pair<Rule>) -> TraitIdent {
    match pair.as_rule() {
        Rule::traitIdent => TraitIdent(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}

fn parse_forall_ident(pair: Pair<Rule>) -> ForallIdent {
    match pair.as_rule() {
        Rule::forallIdent => ForallIdent(pair.as_str().to_string()),
        _ => panic!("{pair}"),
    }
}
