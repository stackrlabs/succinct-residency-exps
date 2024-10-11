#![no_main]

use wasmi::{Engine, Linker, Module, Store, core::Pages};
use bytemuck;

const PAGE_SIZE: u32 = 64 * 1024;
#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
fn binary_search(target: i32, list: Vec<i32>, wasm: &[u8]) -> i32 {
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
    let encoded_list = bytemuck::cast_slice(&list);
    let pages_to_grow = (encoded_list.len() as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, encoded_list).expect("Failed to write to memory");


    let binary_search = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "binary_search")
        .expect("Failed to get typed_func");
    let res = binary_search.call(&mut store, (ptr as i32, list.len() as i32, target)).expect("Failed to call");
    res
}
