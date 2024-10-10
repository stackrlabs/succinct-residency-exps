use clap::Parser;
use wasm::merkelize;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/merkle.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let input_value = json["numLeaves"].as_i64().expect("Failed to parse value from JSON") as i32;
    println!("Input value: {}", input_value);
    let start = std::time::Instant::now();
    merkelize(input_value);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
