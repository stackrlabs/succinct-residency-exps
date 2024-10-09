use sp1_sdk::{ProverClient, SP1Stdin};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

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

    let file = File::open("../../../../inputs/block.json").expect("Failed to open file");
    let mut block = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut block).expect("Failed to read file");

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
