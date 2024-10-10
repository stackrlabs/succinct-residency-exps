use hex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/merkle.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let num_leaves = json["numLeaves"]
        .as_i64()
        .expect("Failed to parse value from JSON") as i32;
    println!("Input value: {}", num_leaves);
    let leaves = (0..num_leaves)
        .map(|i| i.to_string().as_bytes().to_vec())
        .collect::<Vec<_>>();
    // let (prove_merkelize_wrapper, verify_merkelize_wrapper) = guest::build_merkelize_wrapper();
    let summary = guest::analyze_merkelize_wrapper(leaves);
    println!("Trace length: {:?}", summary.trace_len());

    // let (output, proof) = prove_merkelize_wrapper(leaves);
    // let is_valid = verify_merkelize_wrapper(proof);

    // println!("output: {}", hex::encode(output));
    // println!("valid: {}", is_valid);
}
