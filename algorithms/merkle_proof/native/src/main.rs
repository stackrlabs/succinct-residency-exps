use wasm::generate_merkle_proof;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/merkle_proof.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Extract the number from the JSON
    let input = json["numLeaves"]
        .as_u64()
        .expect("Failed to parse numLeaves from JSON") as u32;
    println!("Input numLeaves read from JSON: {}", input);

    let start = std::time::Instant::now();
    generate_merkle_proof(input);
    let duration = start.elapsed();
    
    println!("Time elapsed: {:?}", duration);
}
