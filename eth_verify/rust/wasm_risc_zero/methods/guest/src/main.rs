use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};
use wasmi::core::Pages;
use bincode;

const PAGE_SIZE: u32 = 65536;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let block: Vec<u8> = env::read();
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
    // grow memory to fit the graph
    let block_size = block.len();   
    println!("Size of encoded graph: {}", block_size);
    let pages_to_grow = (block_size as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, &block).expect("Failed to write to memory");
    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);


    let verify_block_start = env::cycle_count();
    let verify_block_wasm = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "verify_block_wasm")
        .expect("Failed to get typed_func");
    let res = verify_block_wasm.call(&mut store, (ptr as i32, block_size as i32)).expect("Failed to call");
    let verify_block_end = env::cycle_count();
    eprintln!("verify_block_wasm cycles: {}", verify_block_end - verify_block_start);

    println!("verify_block_wasm {:?}", res);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
