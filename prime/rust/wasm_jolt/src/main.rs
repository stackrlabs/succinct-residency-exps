use hex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
pub fn main() {
     // Read the JSON file
     let file = File::open("../../../inputs/prime.json").expect("Failed to open input file");
     let reader = BufReader::new(file);
     let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
     let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
 
     // Extract the number from the JSON
     let input = json["number"].as_i64().expect("Failed to parse number from JSON") as i32;
 
     println!("Input number read from JSON: {}", input);
    // let (prove_merkelize_wrapper, verify_merkelize_wrapper) = guest::build_merkelize_wrapper();
    let summary = guest::analyze_is_prime(input, &wasm);
    println!("Trace length: {:?}", summary.trace_len());

    // let (output, proof) = prove_merkelize_wrapper(leaves);
    // let is_valid = verify_merkelize_wrapper(proof);

    // println!("output: {}", hex::encode(output));
    // println!("valid: {}", is_valid);
}
