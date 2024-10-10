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
    let wasm = include_bytes!("../../../wasm/main.wasm").to_vec();

    let file = File::open("../../../../inputs/binary.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let input_value = json["value"].as_i64().expect("Failed to parse value from JSON") as i32;
    let list: Vec<i32> = json["list"]
        .as_array()
        .expect("Failed to parse list from JSON")
        .iter()
        .map(|x| x.as_i64().expect("Failed to parse element as i64") as i32)
        .collect();
    println!("Number to check: {}", input_value);
    println!("List length: {}", list.len());
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&wasm);
    stdin.write(&list);
    stdin.write(&input_value);

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