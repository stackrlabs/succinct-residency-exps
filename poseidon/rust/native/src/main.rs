use wasm::poseidon_hash;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/poseidon.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Extract the number from the JSON
    let input = json["arrayLength"]
        .as_u64()
        .expect("Failed to parse arrayLength from JSON") as u32;
    println!("Input arrayLength read from JSON: {}", input);

    let start = std::time::Instant::now();
    poseidon_hash(input);
    let duration = start.elapsed();
    
    println!("Time elapsed: {:?}", duration);
}
