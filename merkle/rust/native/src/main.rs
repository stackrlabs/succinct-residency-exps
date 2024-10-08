use clap::Parser;
use hex;
use wasm::merkelize_impl;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/merkle.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let input_value = json["numLeaves"].as_i64().expect("Failed to parse value from JSON") as i32;
    println!("Input value: {}", input_value);
    let leaves = (0..input_value)
        .map(|i| i.to_string().as_bytes().to_vec())
        .collect::<Vec<_>>();
    
    let root = merkelize_impl(leaves);

    println!("Merkle Root: {:?}", hex::encode(root));
}


