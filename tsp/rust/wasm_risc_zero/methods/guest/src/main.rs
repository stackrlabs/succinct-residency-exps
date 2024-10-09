use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};
use wasmi::core::Pages;
use bincode;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let graph: Vec<Vec<i32>> = env::read();
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
    let ptr = u32::from(memory.current_pages(&mut store)) * 64 * 1024;
    // grow memory to fit the graph
    let encoded_graph = bincode::serialize(&graph).expect("Failed to encode graph");
    let encoded_graph_size = encoded_graph.len();
    println!("Size of encoded graph: {}", encoded_graph_size);
    let pages_to_grow = (encoded_graph_size as u32 + 65535) / 65536;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, &encoded_graph).expect("Failed to write to memory");
    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);


    let tsp_wasm_start = env::cycle_count();
    let tsp_wasm = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "tsp_wasm")
        .expect("Failed to get typed_func");
    let res = tsp_wasm.call(&mut store, (ptr as i32, encoded_graph_size as i32)).expect("Failed to call");
    let tsp_wasm_end = env::cycle_count();
    eprintln!("tsp_wasm cycles: {}", tsp_wasm_end - tsp_wasm_start);

    println!("tsp_wasm {:?}", res);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
