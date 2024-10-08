use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default();
    let module: Module = Module::from_file(&engine, "./nth_fib.wat")?;

    let mut store = Store::new(
        &engine,
        42 // this is host info that can be accessed from the wasm or in the callbacks
    );

    let instance = Instance::new(&mut store, &module, &[])?;
    let nth_fibonacci = instance.get_typed_func::<u32, u32>(&mut store, "fib")?;

    let results = nth_fibonacci.call(&mut store, 20)?;
    println!("Result -> {}", results);
    Ok(())
}
