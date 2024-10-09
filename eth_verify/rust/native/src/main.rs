use alloy_primitives::FixedBytes;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use tokio;
use wasm::{check_mpt_root, verify_block_hash, verify_block_txs_sigs, Block, Header};

#[tokio::main]
async fn main() {
    // Read JSON file from disk, parse it into Block and Header structs
    let file = File::open("../../../inputs/block.json").expect("Failed to open file");
    let block_json: serde_json::Value =
        serde_json::from_reader(BufReader::new(file)).expect("Failed to parse JSON");
    let block: Block = serde_json::from_value(block_json.clone()).unwrap();
    let header: Header = serde_json::from_value(block_json.clone()).unwrap();

    // Verify MPT root (tx root)
    let result = check_mpt_root(block.clone());
    if !result {
        println!("❌ Tx root does not match!");
        return;
    }
    println!("✅ MPT Tx root matches!");

    // Verify tx signatures
    let result = verify_block_txs_sigs(block.clone());
    if !result {
        println!("❌ Tx sigs verification failed!");
        return;
    }
    println!("✅ Tx sigs verification passed!");

    // Verify block header hash
    let hash_str = block_json["hash"].as_str().unwrap();
    let hash_bytes = hex::decode(&hash_str[2..]).unwrap();
    let result = verify_block_hash(header, FixedBytes::from_slice(&hash_bytes));
    if !result {
        println!("❌ Block hash verification failed!");
        return;
    }
    println!("✅ Block hash matches!");
}
