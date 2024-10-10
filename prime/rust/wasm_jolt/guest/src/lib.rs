#![no_main]

use wasmi::{Engine, Linker, Module, Store};

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
fn is_prime(input: i32, wasm: &[u8]) -> bool {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<i32>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let is_prime = instance
        .get_typed_func::<i32, i32>(&mut store, "is_prime")
        .expect("Failed to get typed_func");
    let res = is_prime.call(&mut store, input).expect("Failed to call");
    res == 1
}
