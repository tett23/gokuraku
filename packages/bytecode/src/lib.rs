mod binary;
mod debug;
mod text;
mod wat;

pub use self::{binary::Ir, debug::DebugSymbol, text::parse, wat::make_wat};

pub struct Vm {
    #[allow(dead_code)]
    memory: Vec<u8>,
    #[allow(dead_code)]
    static_end: usize,
    #[allow(dead_code)]
    addr: usize,
}

pub enum Instruction {
    Push(Word),
    Pop,
    Evaluate,
    Copy(Word),
    Print(Word),
}

pub enum Word {}

#[cfg(test)]
mod tests {
    // use super::*;
}
