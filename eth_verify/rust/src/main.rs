use alloy_consensus;
use alloy_primitives::keccak256;
use alloy_rpc_types::Block;
use alloy_rlp;
use serde_json;

fn main() {
    let file_path = "block_data.json";
    let file_content = std::fs::read_to_string(file_path)
        .expect("Failed to read the file");
    let s = file_content.as_str();
    let block = serde_json::from_str::<Block>(s).unwrap();
    let result = verify_block_hash(block);
    println!("Block hash verification result: {}", result);
}

fn verify_block_hash(block: Block) -> bool {
    let header: alloy_consensus::Header = block.clone().header.try_into().unwrap();
    let recomputed_hash = keccak256(alloy_rlp::encode(&header));
    assert_eq!(recomputed_hash, block.header.hash);
    recomputed_hash == block.header.hash
}
