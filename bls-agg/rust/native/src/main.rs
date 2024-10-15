use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use wasm::{bls_aggregate, Signature, PrivateKey};

fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/bls_agg.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let num_signers = json["numSigners"]
        .as_i64()
        .expect("Failed to parse value from JSON") as u32;
    let private_keys: Vec<_> = (0..num_signers)
        .map(|i| PrivateKey::new(&[i as u8; 32]))
        .collect();

    let message = "message".as_bytes().to_vec();
    // sign messages
    let sigs = private_keys
        .iter()
        .map(|pk| pk.sign(&message))
        .collect::<Vec<Signature>>();
    println!("Number of signers: {}", num_signers);
    let start = std::time::Instant::now();
    bls_aggregate(sigs);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
