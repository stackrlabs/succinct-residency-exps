//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::merkelize_impl;

pub fn main() {
    let leaves = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    println!("cycle-tracker-start: execution");

    let res = merkelize_impl(leaves);
    println!("cycle-tracker-end: execution");
    println!("merkelize: root: {:?}", res);

    sp1_zkvm::io::commit(&res);
}
