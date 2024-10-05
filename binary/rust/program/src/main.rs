//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};

pub fn main() {
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let list = sp1_zkvm::io::read::<Vec<i32>>();
    let target = sp1_zkvm::io::read::<i32>();

    let engine = Engine::default();
    println!("cycle-tracker-start: instantiate wasm");
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    println!("cycle-tracker-end: instantiate wasm");

    let mut linker = <Linker<(&[i32], i32)>>::new(&engine);
    let array_list = list.as_slice();
    let mut store = Store::new(&engine, (array_list, target));

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    println!("cycle-tracker-start: call wasm");
    let binary_search = instance
        .get_typed_func::<(&[i32], i32), i32>(&mut store, "binary_search")
        .expect("Failed to get typed_func");
    let res = binary_search.call(&mut store, (list, target)).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("binary_search {} - {}", target, res);

    sp1_zkvm::io::commit(&res);
}
