use hex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
pub fn main() {
     // Read the JSON file
    let file = File::open("../../../inputs/binary.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();

    // Extract the number from the JSON
    let input_json = json["list"].as_array().expect("Failed to parse list from JSON");
    let input_list: Vec<i32> = input_json.iter().map(|v| v.as_i64().expect("Failed to parse list from JSON") as i32).collect();
    let input_value = json["value"].as_i64().expect("Failed to parse value from JSON") as i32;
    // let (prove_merkelize_wrapper, verify_merkelize_wrapper) = guest::build_merkelize_wrapper();
    let summary = guest::analyze_binary_search(input_value, input_list[..1000].to_vec(), &wasm);
    println!("Trace length: {:?}", summary.trace_len());
}
