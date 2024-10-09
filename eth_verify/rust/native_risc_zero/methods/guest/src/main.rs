use risc0_zkvm::guest::env;
use alloy_primitives::FixedBytes;
use wasm::{check_mpt_root, verify_block_hash, Block, Header};

fn main() {
    let total_cycles = env::cycle_count();

    let block: Block = env::read();

    let txn_root_start = env::cycle_count();
    let root_result = check_mpt_root(block);
    eprintln!("txn root cycles: {}", env::cycle_count() - txn_root_start);

    if !root_result {
        env::commit(&false);
        return;
    }

    let header: Header = env::read();
    let hash_bytes: Vec<u8> = env::read();

    let header_start = env::cycle_count();
    let result = verify_block_hash(header, FixedBytes::from_slice(&hash_bytes));
    eprintln!("header hash verification cycles: {}", env::cycle_count() - header_start);

    env::commit(&result);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
