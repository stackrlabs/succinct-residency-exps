use alloy_primitives::FixedBytes;
use wasm::{check_mpt_root, verify_block_hash, Header, Block};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use tokio;

#[tokio::main]
async fn main() {
    // Read JSON file from disk
    let file = File::open("../../../inputs/block.json").expect("Failed to open file");
    let block_json: serde_json::Value = serde_json::from_reader(BufReader::new(file)).expect("Failed to parse JSON");
    // Deserialize the response to get block and transaction data
    let block: Block = serde_json::from_value(block_json.clone()).unwrap();
    let result = check_mpt_root(block);
    println!("Tx root generated and matched successfully: {}", result);
    if !result {
        println!("Tx root does not match");
        return;
    }

    let header: Header = serde_json::from_value(block_json.clone()).unwrap();

    let hash_str = block_json["hash"].as_str().unwrap();
    let hash_bytes = hex::decode(&hash_str[2..]).unwrap();
    let result = verify_block_hash(header, FixedBytes::from_slice(&hash_bytes));
    println!("Block hash verification result: {}", result);
}

