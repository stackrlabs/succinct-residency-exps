use alloy_consensus::TxEip1559;
use reth_primitives::{Signature};
use serde_json;
use serde;
use alloy_primitives::{B256, Parity};
use eth_verify::{ Header, calculate_transaction_root, calculate_mpt_root, Block, Transaction};
use tokio;
use alloy_rpc_types::{BlockTransactions};
use reth_primitives::{TransactionSigned, Transaction as RethTransaction};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::error::Error;
use rlp::RlpStream;
use hex;

#[derive(serde::Deserialize)]
struct InputBlock {
    #[serde(rename = "header")]
    header: Header,
    #[serde(rename = "hash")]
    expected_hash: B256,
}


#[tokio::main]
async fn main() {
    // Fetch the latest block
    let block = get_block(1800000).await.expect("Failed to fetch block data");
    let result = calculate_mpt_root(block).await;
    println!("Tx root generated and matched successfully: {}", result);
    return;
    // let tx_root = block["result"]["transactionsRoot"].as_str().expect("Failed to get transactions root");
    // println!("Transaction root from block: {:?}", tx_root);

    // // Extract transactions from the block
    // let transactions = block["result"]["transactions"].as_array().expect("Failed to get transactions");


    
    // let header = serde_json::from_str::<Header>(&block_data).unwrap();
    // let expected_hash = header.transactions_root;
    // println!("Expected hash: {:?}", expected_hash);
    // // Save the block data to a file
    // // let mut file = File::create("block_data.json").expect("Failed to create file");
    // // file.write_all(block_data.as_bytes()).expect("Failed to write to file");

    // println!("Block data saved to block_data.json");
    // let file_path = "../../../inputs/block_data.json";
    // let file_content = std::fs::read_to_string(file_path)
    //     .expect("Failed to read the file");
    // let s = file_content.as_str();
    // let input_block = serde_json::from_str::<InputBlock>(s).unwrap();
    // let result = verify_block_hash(input_block.header, input_block.expected_hash);
    // println!("Block hash verification result: {}", result);
}

// Function to fetch block data from Ethereum mainnet
async fn get_block(block_number: u64) -> Result<Block, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://cold-intensive-water.quiknode.pro/7c494f811bd903c29a638b46b60e811d8c29fd81"; // Your Ethereum node URL
    let block_number_hex = format!("0x{:x}", block_number);
    
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [block_number_hex, true], // true to include transactions
        "id": 1,
    });
    
    let response = client.post(url)
        .json(&request_body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Deserialize the response to get block and transaction data
    let block = serde_json::from_value(response["result"].clone())?;
    Ok(block)
}

