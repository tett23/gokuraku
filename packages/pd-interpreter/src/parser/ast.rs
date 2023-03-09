#[derive(Debug)]
pub enum PdsTokens {
    Pds(Pds),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Pds(pub Vec<Assign>);

#[derive(Debug)]
pub enum Expr {
    Assign(Ident, Vec<Ident>, Box<Expr>),
    Literal(Literal),
}

#[derive(Debug)]
pub struct Assign(pub Ident, pub Vec<Ident>, pub Box<Expr>);

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Literal {
    Text(String),
    Int(isize),
}
