use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    // Read in wasm file from disk
    let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the input JSON file
    let file = File::open("../../../inputs/tsp.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let mut graph: Vec<Vec<i32>> =
        serde_json::from_value(json["graph"].clone()).expect("Failed to parse graph from JSON");
    // FIXME: using a lower value cz of memory issues
    graph = graph[0..2].to_vec();
    println!("Input graph length: {:?}", graph.len());

    let summary = guest::analyze_tsp(graph, &wasm);
    println!("Trace length: {:?}", summary.trace_len());
}
