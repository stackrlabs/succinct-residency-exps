use risc0_zkvm::guest::env;
use wasmi::core::Pages;
use wasmi::{Engine, Linker, Module, Store};
use serde_cbor;
use wasm::Inputs;

const PAGE_SIZE: u32 = 64 * 1024;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let aggregate_signature: Vec<u8> = env::read();
    let public_keys: Vec<Vec<u8>> = env::read();
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

    // write list to memory
    let memory = instance
        .get_memory(&store, "memory")
        .expect("Failed to get memory");
    let ptr = u32::from(memory.current_pages(&mut store)) * PAGE_SIZE;
    // grow memory to fit the block
    let inputs = Inputs {
        aggregated_signature: aggregate_signature,
        public_keys: public_keys,
    };
    let serialized_inputs = serde_cbor::to_vec(&inputs).expect("Failed to serialize inputs");
    let inputs_len = serialized_inputs.len();
    let pages_to_grow = (inputs_len as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory
        .grow(
            &mut store,
            Pages::new(pages_to_grow).expect("Failed to grow memory"),
        )
        .expect("Failed to grow memory");
    memory
        .write(&mut store, ptr as usize, &serialized_inputs)
        .expect("Failed to write to memory");
    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);

    let bls_verify_start = env::cycle_count();
    let bls_verify = instance
        .get_typed_func::<(i32, i32), u32>(&mut store, "bls_verify_wasm")
        .expect("Failed to get typed_func");
    let res = bls_verify
        .call(&mut store, (ptr as i32, inputs_len as i32))
        .expect("Failed to call");
    let bls_verify_end = env::cycle_count();
    eprintln!("bls_verify cycles: {}", bls_verify_end - bls_verify_start);
    println!("bls_verify - {}", res);

    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
