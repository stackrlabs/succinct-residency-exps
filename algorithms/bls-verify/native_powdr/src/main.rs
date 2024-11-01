use powdr::Session;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use wasm::{PublicKey, PrivateKey, Serialize};
use hex;

fn main() {
    env_logger::init();

    // Read the JSON file
    let file = File::open("../../../inputs/bls_verify.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let input_value = json["numSigners"]
        .as_i64()
        .expect("Failed to parse value from JSON") as u32;
    let aggregate_signature = json["aggregateSignature"]
        .as_str()
        .expect("Failed to parse value from JSON");
    println!("Input value: {}", input_value);
    let private_keys: Vec<PrivateKey> = (0..input_value)
        .map(|i| PrivateKey::new(&[i as u8; 32]))
        .collect();
    
    let public_keys = private_keys
        .iter()
        .map(|pk| pk.public_key().as_bytes().to_vec())
        .collect::<Vec<_>>();

    // Create a new powdr session to make proofs for the `guest` crate.
    // Store all temporary and final artifacts in `powdr-target`.
    // Write `some_data` to channel 1 and the sum of `some_data` to channel 2.
    // Any serde-serializable type can be written to a channel.
    let mut session = Session::builder()
        .guest_path("./guest")
        .out_path("powdr-target")
        .build()
        .write(1, &hex::decode(aggregate_signature).unwrap())
        .write(2, &public_keys);

    // Fast dry run to test execution.
    session.run();

    // Uncomment to compute the proof.
    // session.prove();
}
