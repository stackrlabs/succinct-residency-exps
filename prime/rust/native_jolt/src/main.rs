use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/prime.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // Extract the number from the JSON
    let input = json["number"].as_i64().expect("Failed to parse number from JSON") as i32;

    println!("Input number read from JSON: {}", input);
    let (prove_is_prime_wrapper, verify_is_prime_wrapper) = guest::build_is_prime_wrapper();
    // let summary = guest::analyze_is_prime_wrapper(input);
    // println!("summary: {:?}", summary.trace_len());

    let (output, proof) = prove_is_prime_wrapper(input);
    let is_valid = verify_is_prime_wrapper(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
