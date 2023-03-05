mod eval_context;
mod vm;

use crate::ast::Expr;

use self::vm::Vm;

pub fn eval() {
    let mut state = Vm::default();
    state.push(Expr::Literal(crate::ast::Literal::Text(
        "value".to_string(),
    )));
    let ret = state.ret();

    dbg!(ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn a() {
        eval();
        assert!(false);
    }
}
