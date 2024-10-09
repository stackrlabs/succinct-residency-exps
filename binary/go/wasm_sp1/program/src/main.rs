//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Caller, Engine, Instance, Linker, Memory, Module, Store};
use bytemuck::cast_slice;

pub fn main() {
    println!("cycle-tracker-start: read inputs");
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let list = sp1_zkvm::io::read::<Vec<i32>>();
    let target = sp1_zkvm::io::read::<i32>();
    println!("cycle-tracker-end: read inputs");
    
    println!("cycle-tracker-start: instantiate wasm");
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let mut linker = <Linker<String>>::new(&engine);
    let mut store = Store::new(&engine, "42".to_string());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    // write list to memory
    let memory = instance.get_memory(&store,"memory").expect("Failed to get memory");
    let ptr = memory.data_size(&mut store) as i32;
    let mut encoded_list = Vec::new();
    for &item in &list {
        encoded_list.extend_from_slice(&item.to_le_bytes());
    }
    println!("encoded_list_len: {:?}", encoded_list.len());
    let memory_size = (encoded_list.len() as u32 + 65535) / 65536; // round up to the nearest 64KiB page range
    memory.grow(&mut store, memory_size).expect("Failed to grow memory");
    memory.write(&mut store, ptr as usize, &encoded_list).expect("Failed to write to memory");
    println!("cycle-tracker-end: instantiate wasm");

    println!("cycle-tracker-start: call wasm");
    let binary_search = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "binarySearch")
        .expect("Failed to get typed_func");
    let res = binary_search.call(&mut store, (ptr, list.len() as i32, target)).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("binary_search {:?} - {:?}", target, res);

    sp1_zkvm::io::commit(&res);
}
