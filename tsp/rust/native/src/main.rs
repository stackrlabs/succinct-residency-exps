use wasm::run_tsp;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("../../../inputs/tsp.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let graph: Vec<Vec<i32>> = serde_json::from_value(json["graph"].clone()).expect("Failed to parse graph from JSON");

    let result = run_tsp(graph);
    println!("{}", result);
}
