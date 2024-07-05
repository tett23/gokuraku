extern crate bytecode;
use bytecode::{make_wat, parse};
use std::env;
use std::fs;
use wasmtime::*;

fn main() -> anyhow::Result<()> {
    dbg!(fs::read_to_string("fixtures/1.pirt")?);
    let wat = env::args()
        .skip(1)
        .map(|arg| fs::read_to_string(&arg))
        .collect::<std::io::Result<String>>()
        .into_iter()
        .map(|ir| parse(&ir))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|item| make_wat(item).pretty_print())
        .collect::<String>();
    println!("{}", &wat);

    let engine = Engine::default();
    let module = Module::new(&engine, wat)?;

    let mut store = Store::new(&engine, ());

    let hello_func = Func::wrap(&mut store, |_caller: Caller<'_, ()>| {
        println!("> Hello");
    });

    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    let run = instance.get_typed_func::<(), ()>(&mut store, "main")?;
    run.call(&mut store, ())?;

    Ok(())
}
