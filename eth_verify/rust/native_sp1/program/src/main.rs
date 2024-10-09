//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_primitives::B256;
use wasm::{check_mpt_root, verify_block_hash, verify_block_txs_sigs, Block, Header};

pub fn main() {
    println!("cycle-tracker-start: input");
    let block = sp1_zkvm::io::read::<Block>();
    let block_header = sp1_zkvm::io::read::<Header>();
    let block_hash = sp1_zkvm::io::read::<B256>();
    println!("cycle-tracker-end: input");
    println!("cycle-tracker-start: execution");
    let res = verify_block_hash(block_header, block_hash);
    let res2 = verify_block_txs_sigs(block.clone());
    let res3 = check_mpt_root(block.clone());
    println!("cycle-tracker-end: execution");
    let full_result = res && res2 && res3;
    println!("Block verification: {:?}", full_result);

    sp1_zkvm::io::commit(&full_result);
}
