use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let signers_count: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let init_start = env::cycle_count();
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);


    let bls_aggregate_start = env::cycle_count();
    let bls_aggregate = instance
        .get_typed_func::<u32, u32>(&mut store, "bls_aggregate")
        .expect("Failed to get typed_func");
    let res = bls_aggregate.call(&mut store, signers_count).expect("Failed to call");
    let bls_aggregate_end = env::cycle_count();
    eprintln!("bls_aggregate cycles: {}", bls_aggregate_end - bls_aggregate_start);

    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
