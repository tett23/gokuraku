use super::Ident;

#[derive(Debug, Clone, Default)]
pub struct EvalContext {
    pub idents: Vec<Ident>,
}

impl EvalContext {
    pub fn new_local_context() -> Self {
        Self::default()
    }
}
