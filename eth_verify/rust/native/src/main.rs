use alloy_primitives::FixedBytes;
use eth_verify::{calculate_mpt_root, verify_block_hash, Header};
use serde::{self, Deserialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use tokio;
use reqwest::Client;
use std::env;
use dotenv;

#[derive(Deserialize)]
struct BlockInfo {
    number: u64,
    hash: String,
}

#[tokio::main]
async fn main() {
    // Read JSON file from disk
    let file = File::open("../../../inputs/block.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let block_info: BlockInfo = serde_json::from_reader(reader).expect("Failed to parse JSON");

    println!("Block number: {}", block_info.number);
    println!("Block hash: {}", block_info.hash);
    // Fetch the latest block
    // Load environment variables from .env file
    dotenv::dotenv().expect("Failed to read .env file");
    // Get the URL from the environment variable
    let url = env::var("ETH_NODE_URL").expect("ETH_NODE_URL not set in .env file");
    let resp = get_block_resp(block_info.number, &url)
        .await
        .expect("Failed to fetch block data");
    // Deserialize the response to get block and transaction data
    let block = serde_json::from_value(resp["result"].clone()).unwrap();
    let result = calculate_mpt_root(block).await;
    println!("Tx root generated and matched successfully: {}", result);
    if !result {
        println!("Tx root does not match");
        return;
    }

    let header: Header = serde_json::from_value(resp["result"].clone()).unwrap();

    let hash_bytes = hex::decode(&block_info.hash[2..]).unwrap();
    let result = verify_block_hash(header, FixedBytes::from_slice(&hash_bytes));
    println!("Block hash verification result: {}", result);
}

// Function to fetch block data from Ethereum mainnet
async fn get_block_resp(block_number: u64, url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = Client::new();
    let block_number_hex = format!("0x{:x}", block_number);

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [block_number_hex, true], // true to include transactions
        "id": 1,
    });

    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}
