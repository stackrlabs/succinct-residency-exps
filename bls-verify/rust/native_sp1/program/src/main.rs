//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::bls_verify;

pub fn main() {
    let num_signers = sp1_zkvm::io::read::<u32>();
    let aggregate_signature = sp1_zkvm::io::read::<Vec<u8>>();
    println!("cycle-tracker-start: execution");

    let res = bls_verify(num_signers, &aggregate_signature);
    println!("cycle-tracker-end: execution");

    sp1_zkvm::io::commit(&res);
}
