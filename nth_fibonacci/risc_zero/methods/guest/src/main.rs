use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};

fn main() {
    // read the input
    let wasm: Vec<u8> = env::read();
    let input: i32 = env::read();
    println!("input: {}", input);

    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<i32>>::new(&engine);
    let mut store = Store::new(&engine, 42);

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let nth_fibonacci = instance
        .get_typed_func::<i32, i32>(&mut store, "fib")
        .expect("Failed to get typed_func");
    let res = nth_fibonacci.call(&mut store, input).expect("Failed to call");

    env::commit(&res);
}
