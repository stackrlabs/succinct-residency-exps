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

    let keccak_n_start = env::cycle_count();
    let keccak_n = instance
        .get_typed_func::<u32, u32>(&mut store, "keccak_n")
        .expect("Failed to get typed_func");
    let res = keccak_n.call(&mut store, input).expect("Failed to call");
    let keccak_n_end = env::cycle_count();
    eprintln!("keccak_n cycles: {}", keccak_n_end - keccak_n_start);
    println!("keccak_n result: {}", res);

    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
