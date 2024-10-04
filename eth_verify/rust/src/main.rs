use alloy_primitives::{keccak256, B256};
use alloy_rlp;
use serde_json;
use alloy_rpc_types::Block;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
struct InputBlock {
    header: Block,
    hash: B256,
}

fn main() {
    let file_path = "../../inputs/block_data.json";
    let file_content = std::fs::read_to_string(file_path)
        .expect("Failed to read the file");
    let s = file_content.as_str();
    let block = serde_json::from_str::<InputBlock>(s).unwrap();
    let result = verify_block(block.header, block.hash);
    println!("Block hash verification result: {}", result);
}

fn verify_block(block: Block, hash: B256) -> bool {
    let header: alloy_consensus::Header = block.clone().header.try_into().unwrap();
    let recomputed_hash = keccak256(alloy_rlp::encode(&header));
    assert_eq!(recomputed_hash, hash);
    recomputed_hash == hash
}
