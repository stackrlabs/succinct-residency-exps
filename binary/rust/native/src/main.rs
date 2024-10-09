use rand;
use wasm::binary_search_impl;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

fn main() {
    let file = File::open("../../../inputs/binary.json").expect("Failed to open config file");    
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");

    let list: Vec<i32> = serde_json::from_value(json["list"].clone()).expect("Unable to parse list");
    let number: i32 = serde_json::from_value(json["value"].clone()).expect("Unable to parse value");

    let found = binary_search_impl(list, number);
    println!("Element found?: {}", found);
}

