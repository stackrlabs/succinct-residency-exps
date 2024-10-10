//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

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

    let args = Args::parse();
    // Read in wasm file from disk
    let wasm = include_bytes!("../../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the JSON file
    let file = File::open("../../../../inputs/binary.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Extract the number from the JSON
    let input_json = json["list"].as_array().expect("Failed to parse list from JSON");
    let input_list: Vec<i32> = input_json.iter().map(|v| v.as_i64().expect("Failed to parse list from JSON") as i32).collect();
    let number_to_check = json["value"].as_i64().expect("Failed to parse value from JSON") as i32;
    println!("Input list: {}", input_list.len());
    println!("Input value: {}", number_to_check);

    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&wasm);
    stdin.write(&input_list);
    stdin.write(&number_to_check);

    if args.execute {
    // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let res = output.read::<i32>();
        println!("Element found?: {}", res);
        println!("Program output: {}", report);
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
