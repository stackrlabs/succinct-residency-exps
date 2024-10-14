use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/merkle_proof.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // Extract the number from the JSON
    let input = json["numLeaves"].as_u64().expect("Failed to parse numLeaves from JSON") as u32;
    println!("Input numLeaves read from JSON: {}", input);
    // Call the generate_merkle_proof_wrapper function which is jolt::provable
    let start = std::time::Instant::now();
    let summary = guest::analyze_generate_merkle_proof_wrapper(input);
    let duration = start.elapsed();
    println!("Trace length: {:?}", summary.trace_len());
    println!("Time elapsed: {:?}", duration);

    // let (prove_generate_merkle_proof_wrapper, verify_generate_merkle_proof_wrapper) = guest::build_generate_merkle_proof_wrapper();
    // let (output, proof) = prove_generate_merkle_proof_wrapper(input);
    // let is_valid = verify_generate_merkle_proof_wrapper(proof);

    // println!("output: {:?}", output);
    // println!("valid: {}", is_valid);
}
