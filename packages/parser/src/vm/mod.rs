use anyhow::anyhow;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    io::{stdout, Write},
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
    stdout: Box<RefCell<dyn Write>>,
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            stack: Vec::new(),
            environment: Environment::default(),
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
    fn write(vm: &mut Vm, expr: Rc<Expr>) {
        vm.push(expr.clone());
        vm.call_top();

        let expr = vm.pop();
        let value = expr.literal_value().to_string();

        let result = write!(vm.stdout.get_mut(), "{value}");
        match result {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }

        vm.push(Rc::new(Expr::Literal(Literal::Unit)))
    }

    pub fn exec(vm: &mut Vm, ident: &Ident, expr: Rc<Expr>) {
        match ident.ident.as_str() {
            "write" => EmbeddedEnvironment::write(vm, expr),
            _ => panic!(),
        }
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
            Expr::ApplyEmbedded(ident, expr) => {
                EmbeddedEnvironment::exec(self, ident, expr.clone());
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
