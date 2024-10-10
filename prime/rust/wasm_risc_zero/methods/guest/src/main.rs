use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let input: i32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let init_start = env::cycle_count();
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let mut linker = <Linker<i32>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);


    let is_prime_start = env::cycle_count();
    let is_prime = instance
        .get_typed_func::<i32, i32>(&mut store, "is_prime")
        .expect("Failed to get typed_func");
    let res = is_prime.call(&mut store, input).expect("Failed to call");
    let is_prime_end = env::cycle_count();
    eprintln!("is_prime cycles: {}", is_prime_end - is_prime_start);

    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
