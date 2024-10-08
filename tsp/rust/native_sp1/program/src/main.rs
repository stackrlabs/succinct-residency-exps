//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::run_tsp;

pub fn main() {
    println!("cycle-tracker-start: input");
    let graph = sp1_zkvm::io::read::<Vec<Vec<i32>>>();
    println!("cycle-tracker-end: input");
    println!("cycle-tracker-start: execution");
    let res = run_tsp(graph);
    println!("cycle-tracker-end: execution");
    println!("tsp: found path: {}", res);

    sp1_zkvm::io::commit(&res);
}
