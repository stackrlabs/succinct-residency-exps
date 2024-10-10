use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/tsp_10.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let graph: Vec<Vec<i32>> = serde_json::from_value(json["graph"].clone()).expect("Failed to parse graph from JSON");
    // let (prove_tsp_wrapper, verify_tsp_wrapper) = guest::build_run_tsp_wrapper();
    let summary = guest::analyze_run_tsp_wrapper(graph);
    println!("Trace length: {:?}", summary.trace_len());

    // let (output, proof) = prove_tsp_wrapper(graph);
    // let is_valid = verify_tsp_wrapper(proof);

    // println!("output: {}", output);
    // println!("valid: {}", is_valid);
}
