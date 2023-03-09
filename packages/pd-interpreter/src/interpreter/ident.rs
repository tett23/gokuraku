#[derive(Debug, Clone)]
pub struct Ident {
    pub id: usize,
    pub name: String,
    pub infix: bool,
    pub arity: usize,
}
