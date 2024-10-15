//! A simple script to generate and verify the proof of a given program.

use clap::Parser;
use serde_json::Value;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::fs::File;
use std::io::BufReader;
use wasm::{PrivateKey, Signature, Serialize};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Read the JSON file
    let file = File::open("../../../../inputs/bls_agg.json").expect("Failed to open input file");
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
        .map(|pk| pk.sign(&message).as_bytes())
        .collect::<Vec<Vec<u8>>>();
    // Serialize the signatures into bytes using CBOR
    println!("Number of signers: {}", num_signers);
    let args = Args::parse();
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&sigs);

    if args.execute {
        // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let result = output.read::<u32>();
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        let proof_bytes = proof.bytes();
        println!("Proof Size: {}", proof_bytes.len());

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
