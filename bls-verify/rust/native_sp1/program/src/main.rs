//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::{bls_verify, PublicKey};

pub fn main() {
    let aggregate_signature = sp1_zkvm::io::read::<Vec<u8>>();
    let public_keys = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    println!("cycle-tracker-start: execution");

    let res = bls_verify(&aggregate_signature, public_keys);
    println!("cycle-tracker-end: execution");

    sp1_zkvm::io::commit(&res);
}
