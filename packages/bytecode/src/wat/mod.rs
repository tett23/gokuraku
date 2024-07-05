mod print;
use crate::{
    binary::{Expr, Instruction, Literal, Signature, Symbol, TypeSymbol},
    Ir,
};

pub fn make_wat(ir: Ir) -> WatExpr {
    dbg!(&ir);
    let wat: WatExpr = ir.into();
    wat
}

#[derive(Debug, Clone)]
pub enum WatExpr {
    Module(Vec<WatExpr>),
    Func(
        WasmSymbol,
        Option<String>,
        Vec<(WasmSymbol, WasmType)>,
        Vec<WasmType>,
        Vec<WatExpr>,
    ),
    Export(String),
    Param(WasmSymbol, WasmType),
    Result(WasmType),
    Call(WasmSymbol, Vec<WatExpr>),
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I64Add,
    LocalGet(WasmSymbol),
    Start(WasmSymbol),
}

#[derive(Debug, Clone)]
enum WasmType {
    I32,
    I64,
    F32,
    F64,
    V128,
    AnyFunc,
}

#[derive(Debug, Clone)]
struct WasmSymbol(String);

impl From<Ir> for WatExpr {
    fn from(ir: Ir) -> Self {
        match ir {
            Ir::Module(module) => {
                let executable = module.0.iter().any(|ir| matches!(ir, Ir::FunctionSymbol(Symbol(sym), _, _) if sym.as_str() == "main"));
                let mut wasm_module = module
                    .0
                    .into_iter()
                    .map(|ir| ir.into())
                    .collect::<Vec<WatExpr>>();
                if executable {
                    wasm_module.push(WatExpr::Start(WasmSymbol("main".into())));
                };
                WatExpr::Module(wasm_module)
            }
            Ir::FunctionSymbol(ref name, signature, expr) => {
                let Signature(args, ret) = signature;

                WatExpr::Func(
                    name.into(),
                    Some(name.as_str().to_string()),
                    args.into_iter()
                        .map(|(ref name, ty)| (name.into(), ty.into()))
                        .collect::<Vec<_>>(),
                    vec![ret.into()],
                    expr.into(),
                )
            }
            // Ir::Expr(expr) => expr.into(),
            _ => unreachable!(),
        }
    }
}

impl From<TypeSymbol> for WasmType {
    fn from(value: TypeSymbol) -> Self {
        match value.0.as_str() {
            "Int" => WasmType::I64,
            "()" => WasmType::I32,
            _ => unreachable!(),
        }
    }
}

impl From<Expr> for Vec<WatExpr> {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::FnCall(symbol, args) => vec![WatExpr::Call(
                symbol.as_ref().to_owned().try_into().unwrap(),
                args.into_iter()
                    .flat_map(Vec::<WatExpr>::from)
                    .collect::<Vec<_>>(),
            )],
            Expr::InstCall(inst, args) => {
                let inst = match inst {
                    Instruction::Add => args
                        .iter()
                        .flat_map(|a| Vec::<WatExpr>::from(a.clone()))
                        .chain(vec![WatExpr::I64Add])
                        .collect::<Vec<WatExpr>>(),
                };

                inst
            }
            Expr::Literal(literal) => {
                vec![literal.into()]
            }
            Expr::SymbolRef(ref name) => {
                vec![WatExpr::LocalGet(name.into())]
            }
        }
    }
}

impl TryFrom<Expr> for WasmSymbol {
    type Error = &'static str;

    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        match expr {
            Expr::SymbolRef(name) => Ok(WasmSymbol(name.to_string())),
            _ => Err(""),
        }
    }
}

impl From<Literal> for WatExpr {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Int(value) => {
                WatExpr::I64Const(i64::try_from(value).expect("Failed to parse literal as i64"))
            }
            Literal::Float(value) => WatExpr::F64Const(value),
            _ => unimplemented!(),
        }
    }
}

impl From<&Symbol> for WasmSymbol {
    fn from(Symbol(s): &Symbol) -> Self {
        WasmSymbol(s.to_string())
    }
}
