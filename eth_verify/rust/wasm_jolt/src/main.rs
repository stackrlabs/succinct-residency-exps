use std::fs::File;
use std::io::{BufReader, Read};

pub fn main() {
    // Read in wasm file from disk
    let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the input JSON file
    let file = File::open("../../../inputs/block.json").expect("Failed to open file");
    let mut encoded_block: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut encoded_block).expect("Failed to read file");
    println!("Size of encoded block: {}", encoded_block.len());
    // Invoke the jolt program
    let summary = guest::analyze_verify_block(encoded_block, &wasm);
    println!("Trace length: {:?}", summary.trace_len());
}
