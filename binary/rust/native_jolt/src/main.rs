use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/binary.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Extract the number from the JSON
    let input_json = json["list"].as_array().expect("Failed to parse list from JSON");
    let input_list: Vec<i32> = input_json.iter().map(|v| v.as_i64().expect("Failed to parse list from JSON") as i32).collect();
    let input_value = json["value"].as_i64().expect("Failed to parse value from JSON") as i32;
    println!("Input list: {}", input_list.len());
    println!("Input value: {}", input_value);

    // let (prove_binary_search_wrapper, verify_binary_search_wrapper) = guest::build_binary_search_wrapper();
    let summary = guest::analyze_binary_search_wrapper(input_list, input_value);
    println!("Trace length: {:?}", summary.trace_len());

    // let (output, proof) = prove_binary_search_wrapper(input_list, input_value);
    // let is_valid = verify_binary_search_wrapper(proof);

    // println!("output: {}", output);
    // println!("proof valid: {}", is_valid);
}
