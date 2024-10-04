
use serde_json;
use serde;
use alloy_primitives::B256;
use eth_verify::{verify_block_hash, Header};

#[derive(serde::Deserialize)]
struct InputBlock {
    #[serde(rename = "header")]
    header: Header,
    #[serde(rename = "hash")]
    expected_hash: B256,
}

fn main() {
    let file_path = "../../inputs/block_data.json";
    let file_content = std::fs::read_to_string(file_path)
        .expect("Failed to read the file");
    let s = file_content.as_str();
    let input_block = serde_json::from_str::<InputBlock>(s).unwrap();
    let result = verify_block_hash(input_block.header, input_block.expected_hash);
    println!("Block hash verification result: {}", result);
}

