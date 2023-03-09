use crate::interpreter::Ident;

#[derive(Debug, Clone)]
pub enum PdAst {}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Function(IdentName),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Apply(IdentName),
}

pub type IdentName = String;

pub struct Function {
    pub ident: Ident,
}

// impl Expr {
//     pub fn eval(&mut self) -> Value {
//         match self {
//             Expr::Literal(value) => Value(value.clone()),
//             Expr::Apply(value) => todo!(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Literal {
    Text(String),
    Int(isize),
}
