//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use wasm::Header;
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

    // Read in wasm file from disk
    let wasm = include_bytes!("../../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();

    let file_path = "../../../../inputs/block_data.json";
    let file_content = std::fs::read_to_string(file_path)
        .expect("Failed to read the file");
    let block = file_content.as_bytes().to_vec();
    // let input_block = serde_json::from_str::<InputBlock>(s).unwrap();

    let args = Args::parse();
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&wasm);
    stdin.write(&block);

    if args.execute {
    // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let result = output.read::<bool>();
        println!("block verified: {}", result);
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
