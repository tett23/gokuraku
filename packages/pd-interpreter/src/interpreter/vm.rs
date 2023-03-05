use anyhow::anyhow;

use super::eval_context::EvalContext;
use crate::ast::{Expr, Literal, Value};

#[derive(Debug, Clone, Default)]
pub struct Vm {
    stack: Vec<Expr>,
    context_stack: Vec<EvalContext>,
}

impl Vm {
    pub fn push(&mut self, value: Expr) {
        self.stack.push(value);
    }

    pub fn ret(mut self) -> Value {
        if self.stack.len() != 1 {
            panic!("{}", anyhow!("stack error"));
        }

        self.stack.pop().unwrap().eval()
    }
}
