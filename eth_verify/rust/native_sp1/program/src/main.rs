//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);
use wasm::{calculate_mpt_root, verify_block_hash, Header, Block};
use alloy_primitives::{B256};

pub fn main() {
    println!("cycle-tracker-start: input");
    let block = sp1_zkvm::io::read::<Block>();
    let block_header = sp1_zkvm::io::read::<Header>();
    let block_hash = sp1_zkvm::io::read::<B256>();
    println!("cycle-tracker-end: input");
    println!("cycle-tracker-start: execution");
    let res = verify_block_hash(block_header, block_hash);
    let res2 = calculate_mpt_root(block);
    println!("cycle-tracker-end: execution");
    let full_result = res && res2;
    println!("Block verification: {:?}", full_result);

    sp1_zkvm::io::commit(&full_result);
}
