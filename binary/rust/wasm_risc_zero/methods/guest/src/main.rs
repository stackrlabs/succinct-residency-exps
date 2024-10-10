use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};
use wasmi::core::Pages;
use bytemuck;
const PAGE_SIZE: u32 = 65536;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let wasm: Vec<u8> = env::read();
    let list: Vec<i32> = env::read();
    let target: i32 = env::read();
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
    let encoded_list = bytemuck::cast_slice(&list);
    let pages_to_grow = (encoded_list.len() as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory.grow(&mut store, Pages::new(pages_to_grow).expect("Failed to grow memory")).unwrap();
    memory.write(&mut store, ptr as usize, encoded_list).expect("Failed to write to memory");
    let init_end = env::cycle_count();
    eprintln!("instantiate wasm cycles: {}", init_end - init_start);


    let is_binary_search_start = env::cycle_count();
    let binary_search = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "binary_search")
        .expect("Failed to get typed_func");
    let res = binary_search.call(&mut store, (ptr as i32, list.len() as i32, target)).expect("Failed to call");
    let is_binary_search_end = env::cycle_count();
    eprintln!("is_binary_search cycles: {}", is_binary_search_end - is_binary_search_start);

    println!("binary_search {:?} - {:?}", target, res);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
