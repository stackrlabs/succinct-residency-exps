//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};

pub fn main() {
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let leaves = sp1_zkvm::io::read::<Vec<Vec<u8>>>();

    let engine = Engine::default();
    println!("cycle-tracker-start: instantiate wasm");
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let mut linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    // write list to memory
    let memory = instance.get_memory(&store,"memory").expect("Failed to get memory");
    let ptr = memory.data_size(&mut store) as i32;
    memory.grow(&mut store, leaves.len() as u32).expect("Failed to grow memory");
    memory.write(&mut store, ptr as usize, bytemuck::cast_slice(&leaves)).expect("Failed to write to memory");
    println!("cycle-tracker-end: instantiate wasm");

    println!("cycle-tracker-start: call wasm");
    let binary_search = instance
        .get_typed_func::<<Vec<Vec<u8>>, i32>, bool>(&mut store, "binary_search")
        .expect("Failed to get typed_func");
    let res = binary_search.call(&mut store, leaves).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("binary_search {} - {}", target, res);

    sp1_zkvm::io::commit(&res);
}
