#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
extern crate alloc;
use alloc::vec;
use nexus_rt::{read_private_input};
use wasmi::{Engine, Linker, Module, Store};

#[nexus_rt::profile]
fn nth_prime_exec(n: u32, wasm: &[u8]) -> u64 {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<u64>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let nth_prime = instance
        .get_typed_func::<u64, u64>(&mut store, "nth_prime")
        .expect("Failed to get typed_func");
    let res = nth_prime.call(&mut store, input).expect("Failed to call");
    println!("The {}th prime is {}.", input, res);
    res
}

 
#[nexus_rt::main]
fn main() {
    let input = read_private_input::<(u32, &[u8])>().expect("failed to read input");
 
    let mut res: u64 = 0;
    if let Ok((n, wasm)) = input {
        res = nth_prime_exec(n, wasm);
        assert_eq!(541, res);
    } else {
        println!("No private input provided...");
    }
}
