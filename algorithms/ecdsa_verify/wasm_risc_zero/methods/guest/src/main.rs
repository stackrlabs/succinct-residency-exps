use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let input: u32 = env::read();
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

    let ecdsa_verify_n_start = env::cycle_count();
    let ecdsa_verify_n = instance
        .get_typed_func::<u32, u32>(&mut store, "ecdsa_verify_n")
        .expect("Failed to get typed_func");
    let res = ecdsa_verify_n.call(&mut store, input).expect("Failed to call");
    let ecdsa_verify_n_end = env::cycle_count();
    eprintln!("ecdsa_verify_n cycles: {}", ecdsa_verify_n_end - ecdsa_verify_n_start);
    println!("ecdsa_verify_n result: {}", res);

    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
