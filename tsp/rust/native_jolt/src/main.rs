use hex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/tsp.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let graph: Vec<Vec<i32>> = serde_json::from_value(json["graph"].clone()).expect("Failed to parse graph from JSON");
    // let (prove_merkelize_wrapper, verify_merkelize_wrapper) = guest::build_merkelize_wrapper();
    let summary = guest::analyze_run_tsp_wrapper(graph);
    println!("summary: {:?}", summary.trace_len());

    // let (output, proof) = prove_merkelize_wrapper(leaves);
    // let is_valid = verify_merkelize_wrapper(proof);

    // println!("output: {}", hex::encode(output));
    // println!("valid: {}", is_valid);
}
