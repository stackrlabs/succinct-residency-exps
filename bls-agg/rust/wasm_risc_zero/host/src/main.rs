// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{GUEST_CODE_FOR_BLS_AGG_ELF, GUEST_CODE_FOR_BLS_AGG_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use bincode;
use std::fs;

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let wasm = include_bytes!("../../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();

    // Read the JSON file
    let file = File::open("../../../inputs/bls_agg.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let input_value = json["numSigners"].as_u64().expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", input_value);

    let env = ExecutorEnv::builder()
        .write(&wasm).unwrap()
        .write(&input_value).unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove_with_opts(env, GUEST_CODE_FOR_BLS_AGG_ELF, &ProverOpts::groth16()).unwrap();

    let receipt = prove_info.receipt;
    let _output: u32 = receipt.journal.decode().unwrap();
    println!("BLS Agg: {}", _output);

    let output_path = PathBuf::from("proof.bin");
    let receipt_data = bincode::serialize(&receipt).unwrap();
    fs::write(output_path, receipt_data).expect("Failed to write to output file");
}
