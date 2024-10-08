//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::merkelize_impl;

pub fn main() {
    let num_leaves = sp1_zkvm::io::read::<i32>();
    println!("cycle-tracker-start: execution");
    let leaves = (0..num_leaves)
        .map(|i| i.to_string().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let res = merkelize_impl(leaves);
    println!("cycle-tracker-end: execution");
    println!("merkelize: root: {:?}", res);

    sp1_zkvm::io::commit(&res);
}
