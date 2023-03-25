#![feature(generators, generator_trait)]

pub mod ast;
pub mod ir;
mod prose_down;
mod prose_down_script;
// pub mod vm;

pub use self::ir::type_check;
pub use self::prose_down::parse as prose_down_parse;
pub use self::prose_down_script::parse as prose_down_script_parse;
pub use self::prose_down_script::run as prose_down_script_run;
