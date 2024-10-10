use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};
use wasmi::core::Pages;
use bincode;

const PAGE_SIZE: u32 = 65536;
fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let leaves: Vec<Vec<u8>> = env::read();
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
    let memory = instance.get_memory(&store,"memory").expect("Failed to get memory");
    let ptr = u32::from(memory.current_pages(&mut store)) * PAGE_SIZE;
    // grow memory to fit the leaves
    let encoded_leaves = bincode::serialize(&leaves).expect("Failed to encode leaves");
    let encoded_leaves_size = encoded_leaves.len();
    println!("Size of encoded leaves: {}", encoded_leaves_size);
    let pages_to_grow = (encoded_leaves_size as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, &encoded_leaves).expect("Failed to write to memory");
    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);


    let merkelize_start = env::cycle_count();
    let merkelize = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "merkelize")
        .expect("Failed to get typed_func");
    let res = merkelize.call(&mut store, (ptr as i32, encoded_leaves_size as i32)).expect("Failed to call");
    let merkelize_end = env::cycle_count();
    eprintln!("merkelize cycles: {}", merkelize_end - merkelize_start);

    println!("merkelized {:?}", res);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
