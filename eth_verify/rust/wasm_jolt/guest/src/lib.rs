#![no_main]

use wasmi::{Engine, Linker, Module, Store, core::Pages};

const PAGE_SIZE: u32 = 64 * 1024;

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
pub fn verify_block(encoded_block: Vec<u8>, wasm: &[u8]) -> i32 {
    // init Wasm
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
    // grow memory to fit the block (block is already encoded)
    let encoded_block_size = encoded_block.len();   
    println!("Size of encoded block: {}", encoded_block_size);
    let pages_to_grow = (encoded_block_size as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, &encoded_block).expect("Failed to write to memory");

    // Call the verify_block_wasm function in the wasm
    let verify_block_wasm = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "verify_block_wasm")
        .expect("Failed to get typed_func");
    let res = verify_block_wasm.call(&mut store, (ptr as i32, encoded_block_size as i32)).expect("Failed to call");
    println!("verify_block_wasm: {:?}", res);

    res
}
