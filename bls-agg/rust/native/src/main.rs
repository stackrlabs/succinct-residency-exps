use clap::Parser;
use wasm::bls_aggregate;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/bls_agg.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let input_value = json["numSigners"].as_i64().expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", input_value);
    bls_aggregate(input_value);
}
