//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};
use bincode;

pub fn main() {
    println!("cycle-tracker-start: read inputs");
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let leaves = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    println!("cycle-tracker-end: read inputs");

    println!("cycle-tracker-start: instantiate wasm");
    let engine = Engine::default();
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

    // grow memory to fit the leaves
    let encoded_leaves = bincode::serialize(&leaves).expect("Failed to encode leaves");
    let encoded_leaves_size = encoded_leaves.len();
    println!("Size of encoded leaves: {}", encoded_leaves_size);
    let memory_size = (encoded_leaves_size as u32 + 65535) / 65536; // round up to the nearest 64KiB page range
    memory.grow(&mut store, memory_size).expect("Failed to grow memory");
    memory.write(&mut store, ptr as usize, &encoded_leaves).expect("Failed to write to memory");
    println!("cycle-tracker-end: instantiate wasm");

    println!("cycle-tracker-start: call wasm");
    let binary_search = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "merkelize")
        .expect("Failed to get typed_func");
    let res = binary_search.call(&mut store, (ptr, encoded_leaves_size as i32)).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("merkelized - {}", res);

    sp1_zkvm::io::commit(&res);
}
