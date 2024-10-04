//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);
use eth_verify::{verify_block_hash, Header};
use alloy_primitives::{B256};

pub fn main() {
    println!("cycle-tracker-start: input");
    let block_header = sp1_zkvm::io::read::<Header>();
    let block_hash = sp1_zkvm::io::read::<B256>();
    println!("cycle-tracker-end: input");
    println!("cycle-tracker-start: execution");
    let res = verify_block_hash(block_header, block_hash);
    println!("cycle-tracker-end: execution");
    println!("tsp-shortest-path: {:?}", res);

    sp1_zkvm::io::commit(&res);
}
