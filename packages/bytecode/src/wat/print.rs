use super::{WasmSymbol, WasmType, WatExpr};
use std::fmt;

enum PrintFormat {
    Wat,
    WatPretty,
}

struct WatPrinter {
    expr: WatExpr,
    format: PrintFormat,
    indent: usize,
}

impl WatExpr {
    pub fn print(&self) -> String {
        WatPrinter {
            expr: self.clone(),
            format: PrintFormat::WatPretty,
            indent: 0,
        }
        .to_string()
    }

    pub fn pretty_print(&self) -> String {
        self.print_with_indent(0)
    }

    fn print_with_indent(&self, indent: usize) -> String {
        WatPrinter {
            expr: self.clone(),
            format: PrintFormat::Wat,
            indent,
        }
        .to_string()
    }
}

impl fmt::Display for WatPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let WatPrinter {
            expr,
            format,
            indent,
        } = self;

        match expr {
            WatExpr::Module(exprs) => {
                write!(
                    f,
                    "(module\n{}\n)",
                    exprs
                        .iter()
                        .map(|expr| expr.print_with_indent(indent + 1))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            WatExpr::Func(name, export_name, args, ret, exprs) => {
                dbg!(&name, &export_name);
                write!(f, "{}", "    ".repeat(*indent))?;
                write!(
                    f,
                    "(func {} {} {} {}",
                    name,
                    export_name
                        .clone()
                        .map(|a| WatExpr::Export(a.clone()).print())
                        .unwrap_or("".to_string()),
                    args.iter()
                        .map(|(name, ty)| WatExpr::Param(name.clone(), ty.clone()).print())
                        .collect::<Vec<String>>()
                        .join(" "),
                    ret.iter()
                        .map(|a| WatExpr::Result(a.clone()).print())
                        .collect::<Vec<String>>()
                        .join(" "),
                )?;
                write!(f, "\n")?;
                write!(
                    f,
                    "{}",
                    exprs
                        .iter()
                        .map(|expr| expr.print_with_indent(indent + 1))
                        .collect::<Vec<String>>()
                        .join("\n")
                )?;
                write!(f, "\n")?;
                write!(f, "{})", "    ".repeat(*indent))
            }
            WatExpr::Param(name, ty) => {
                write!(f, "(param {} {})", name, ty)
            }
            WatExpr::Result(ty) => {
                write!(f, "(result {})", ty)
            }
            WatExpr::Export(name) => {
                write!(f, "(export \"{}\")", name)
            }
            WatExpr::Call(name, args) => {
                write!(f, "{}", "    ".repeat(*indent))?;
                write!(
                    f,
                    "(call {} {})",
                    name,
                    args.iter()
                        .map(|a| a.print())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            WatExpr::I64Const(val) => {
                write!(f, "(i64.const {})", val)
            }
            WatExpr::I64Add => {
                write!(f, "{}", "    ".repeat(*indent))?;
                write!(f, "(i64.add)")
            }
            WatExpr::LocalGet(symbol) => {
                write!(f, "{}", "    ".repeat(*indent))?;
                write!(f, "(local.get {})", symbol)
            }
            WatExpr::Start(symbol) => {
                // write!(f, "(start {})", symbol)
                Ok(())
            }
            _ => {
                dbg!(expr);
                unimplemented!()
            }
        }
    }
}

impl fmt::Display for WasmSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl fmt::Display for WasmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmType::I32 => write!(f, "i32"),
            WasmType::I64 => write!(f, "i64"),
            WasmType::F32 => write!(f, "f32"),
            WasmType::F64 => write!(f, "f64"),
            WasmType::V128 => write!(f, "v128"),
            WasmType::AnyFunc => write!(f, "anyfunc"),
        }
    }
}
