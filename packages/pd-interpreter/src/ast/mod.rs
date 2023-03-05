#[derive(Debug, Clone)]
pub enum PdAst {}

#[derive(Debug, Clone)]
pub struct Value(pub Literal);

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
}

impl Expr {
    pub fn eval(self) -> Value {
        match self {
            Expr::Literal(value) => Value(value.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Text(String),
}
