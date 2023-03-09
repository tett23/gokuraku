use crate::interpreter::{EvalContext, Ident};

pub fn prelude_context() -> EvalContext {
    EvalContext {
        idents: vec![Ident {
            id: 1,
            name: "+".to_string(),
            infix: true,
            arity: 2,
        }],
    }
}
