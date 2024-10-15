//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::{bls_aggregate, Signature, Serialize};

pub fn main() {
    println!("cycle-tracker-start: input");
    let sig_bytes = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    let sigs = sig_bytes.iter().map(|sig| Signature::from_bytes(&sig).unwrap()).collect();
    println!("cycle-tracker-end: input");
    
    println!("cycle-tracker-start: execution");
    let res = bls_aggregate(sigs);
    println!("cycle-tracker-end: execution");

    sp1_zkvm::io::commit(&res);
}
