use anyhow::anyhow;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    io::{stdout, Write},
    ops::DerefMut,
    rc::Rc,
};

use crate::ir::Literal;
use {
    crate::ir,
    ir::{Expr, Function, Ident, Module},
};

pub struct Vm {
    stack: Vec<Rc<Expr>>,
    environment: Environment,
    embedded: EmbeddedEnvironment,
    stdout: Box<RefCell<dyn Write>>,
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            stack: Vec::new(),
            environment: Environment::default(),
            embedded: EmbeddedEnvironment(),
            stdout: Box::new(RefCell::new(stdout())),
        }
    }
}

impl Debug for Vm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Default)]
pub struct EmbeddedEnvironment();
impl EmbeddedEnvironment {
    pub fn write(&self, mut out: impl Write, value: &str) {
        write!(out, "{value}");
    }
}

#[derive(Debug, Default)]
struct Environment(BTreeMap<Ident, Rc<Function>>);

impl Environment {
    fn find_by_name(&self, ident: &Ident) -> Option<Rc<ir::Expr>> {
        let idents = self.0.values().find(|f| &f.ident_name == ident);

        idents.map(|f| f.as_ref().expr.clone())
    }
}

impl Vm {
    pub fn push(&mut self, value: Rc<Expr>) {
        match value {
            value => self.stack.push(value),
        }
    }

    pub fn pop(&mut self) -> Rc<Expr> {
        match self.stack.pop() {
            Some(v) => v,
            None => panic!("{}", anyhow!("empty stack")),
        }
    }

    pub fn ret(&mut self) -> Rc<Expr> {
        if self.stack.len() != 1 {
            panic!("{}", anyhow!("stack error"));
        }

        self.stack.pop().unwrap()
    }

    pub fn call_top(&mut self) {
        let inst = match self.stack.pop() {
            Some(inst) => inst,
            None => panic!("{}", anyhow!("empty stack")),
        };

        match inst.as_ref() {
            Expr::Apply(_v) => {
                todo!()
            }
            Expr::ApplyEmbedded(ident, expr) if ident.ident == "write" => {
                self.push(expr.clone());
                self.call_top();
                let expr = self.pop();

                self.embedded
                    .write(self.stdout.get_mut(), &expr.literal_value().to_string());

                self.push(Rc::new(Expr::Literal(Literal::Unit)))
            }
            Expr::ApplyEmbedded(_, _) => panic!("{}", anyhow!("undefined embedded")),
            Expr::Literal(_) => self.push(inst),
        }
    }

    pub fn load_module(&mut self, module: Module) {
        module.values.into_iter().for_each(|f| {
            self.environment.0.insert(f.ident.clone(), Rc::new(f));
        })
    }

    pub fn run(&mut self, module: Module) -> Rc<Expr> {
        self.load_module(module);
        let e = self.environment.find_by_name(&"main".into());

        self.push(e.unwrap());
        self.call_top();
        self.ret()
    }
}
