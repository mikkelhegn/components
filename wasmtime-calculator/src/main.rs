use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

wasmtime::component::bindgen!("calculator" in "../calculator/wit/calculator.wit");

fn main() -> wasmtime::Result<()> {

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, 0);

    let component = Component::from_file(&engine, "calculator-add.wasm")?;

    let (calculator, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    let res = calculator.call_calc(&mut store)?;

    println!("Result is {:?}", res);

    Ok(())
}
