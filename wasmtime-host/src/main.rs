use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

wasmtime::component::bindgen!("calculator" in "../calc/wit/calc.wit");

fn main() -> wasmtime::Result<()> {

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, 0);

    let component = Component::from_file(&engine, "../calc/calc-component.wasm")?;

    let (calculator, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    let args: Vec<String> = std::env::args().collect();

    let a: u32 = args[1].trim().parse()?;
    let b: u32 = args[2].trim().parse()?;

    let res = calculator.call_add(&mut store, a, b)?;

    println!("Result is {:?}", res);

    Ok(())
}
