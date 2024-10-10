#![no_main]

use wasm::{check_mpt_root, verify_block_hash, verify_block_txs_sigs, Block, Header};
use alloy_primitives::B256;
use hex;

#[jolt::provable(stack_size = 100000000000, memory_size = 10000000)]
fn verify_block(expected_hash: B256, header: Header) -> bool {
    let res = verify_block_hash(header, expected_hash);
    // let res2 = verify_block_txs_sigs(block.clone());
    // let res3 = check_mpt_root(block.clone());
    true
}
