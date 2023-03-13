use anyhow::anyhow;
use std::{collections::BTreeMap, rc::Rc};
use {
    crate::ir,
    ir::{Expr, Function, Ident, Module},
};

#[derive(Debug, Default)]
pub struct Vm {
    stack: Vec<Rc<Expr>>,
    environment: Environment,
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
