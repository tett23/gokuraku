mod eval_context;
mod ident;
mod local_context;
mod vm;

pub(crate) use eval_context::EvalContext;
pub(crate) use ident::Ident;
pub(crate) use vm::Vm;

use crate::{
    ast::{Expr, Literal, Value},
    prelude::prelude_context,
};

pub fn eval() -> Value {
    let mut state = Vm::default();
    state.context_stack.push(prelude_context());
    state.context_stack.push(EvalContext::new_local_context());

    {
        state.push(Value::Literal(Literal::Int(1)));
        state.push(Value::Literal(Literal::Int(2)));
        // state.push(Value::FunctionName("+".to_string()));
    }

    state.ret()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn a() {
        dbg!(eval());
        assert!(false);
    }
}
