use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    // Read in wasm file from disk
    let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the input JSON file
    // let file = File::open("../../../inputs/merkle.json").expect("Failed to open input file");
    // let reader = BufReader::new(file);
    // let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // let input_value = json["numLeaves"].as_i64().expect("Failed to parse value from JSON") as i32;
    // FIXME: using a lower value cz of memory issues
    let input_value = 5;
    println!("Input value: {}", input_value);

    let summary = guest::analyze_merkelize(input_value, &wasm);
    println!("Trace length: {:?}", summary.trace_len());
}
