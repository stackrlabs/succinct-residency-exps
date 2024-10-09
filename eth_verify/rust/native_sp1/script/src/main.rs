//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use rand::Rng;
use wasm::{Header, Block};
use alloy_primitives::B256;
use serde;

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
    let file = File::open("../../../../inputs/block.json").expect("Failed to open file");
    let block_json: serde_json::Value = serde_json::from_reader(BufReader::new(file)).expect("Failed to parse JSON");
    // Deserialize the response to get block and transaction data
    let block: Block = serde_json::from_value(block_json.clone()).unwrap();
    let header: Header = serde_json::from_value(block_json.clone()).unwrap();

    let hash_str = block_json["hash"].as_str().unwrap();
    let expected_hash = hex::decode(&hash_str[2..]).unwrap();

    let args = Args::parse();
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&block);
    stdin.write(&header);
    stdin.write(&expected_hash);

    if args.execute {
    // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let result = output.read::<bool>();
        println!("block verification: {}", result);
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
