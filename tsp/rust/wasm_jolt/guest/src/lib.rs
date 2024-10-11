#![no_main]

use wasmi::{Engine, Linker, Module, Store, core::Pages};
use bincode;

const PAGE_SIZE: u32 = 64 * 1024;

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
pub fn tsp(graph: Vec<Vec<i32>>, wasm: &[u8]) -> i32 {
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
    // grow memory to fit the graph
    let encoded_graph = bincode::serialize(&graph).expect("Failed to encode graph");
    let encoded_graph_size = encoded_graph.len();
    println!("Size of encoded graph: {}", encoded_graph_size);
    let pages_to_grow = (encoded_graph_size as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, &encoded_graph).expect("Failed to write to memory");

    // Call the tsp_wasm function in the wasm
    let tsp_wasm = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "tsp_wasm")
        .expect("Failed to get typed_func");
    let res = tsp_wasm.call(&mut store, (ptr as i32, encoded_graph_size as i32)).expect("Failed to call");
    println!("tsp_wasm: {:?}", res);

    res
}
