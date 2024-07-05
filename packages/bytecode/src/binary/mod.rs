use std::fmt;

#[derive(Debug, Clone)]
pub enum Ir {
    Module(Module),
    FunctionSymbol(Symbol, Signature, Expr),
    ArgSymbol(Symbol, TypeSymbol),
    Expr(Expr),
    Literal(Literal),
    Instruction(Instruction),
}

#[derive(Debug, Clone)]
pub struct Module(pub Vec<Ir>);

#[derive(Debug, Clone)]
pub struct Signature(pub Vec<(Symbol, TypeSymbol)>, pub TypeSymbol);

#[derive(Debug, Clone)]
pub struct Symbol(pub String);

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct TypeSymbol(pub String);

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(isize),
    Float(f64),
    String(String),
    Unit,
}

#[derive(Debug, Clone)]
pub enum Expr {
    FnCall(Box<Expr>, Vec<Expr>),
    InstCall(Instruction, Vec<Expr>),
    Literal(Literal),
    SymbolRef(Symbol),
}
