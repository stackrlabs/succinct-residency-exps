use hex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use wasm::{Header, Block};
use alloy_primitives::B256;

pub fn main() {
    let file = File::open("../../../inputs/block.json").expect("Failed to open file");
    // Deserialize the response to get block and transaction data
    let block_json: Value = serde_json::from_reader(BufReader::new(file)).expect("Failed to parse JSON");
    let block: Block = serde_json::from_value(block_json.clone()).unwrap();
    let header: Header = serde_json::from_value(block_json.clone()).unwrap();

    let hash_str = block_json["hash"].as_str().unwrap();
    let expected_hash = hex::decode(&hash_str[2..]).unwrap();
    let expected_hash = B256::from_slice(&expected_hash);

    let summary = guest::analyze_verify_block(expected_hash, header);
    println!("Trace length: {:?}", summary.trace_len());

    // let (output, proof) = prove_merkelize_wrapper(leaves);   
    // let is_valid = verify_merkelize_wrapper(proof);

    // println!("output: {}", hex::encode(output));
    // println!("valid: {}", is_valid);
}
