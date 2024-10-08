//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use rand;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

// The arguments for the command.
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
    // let mut graph = vec![vec![0; 15]; 15];
    // for i in 0..15 {
    //     for j in i+1..15 {
    //         let distance = rand::random::<u32>() % 100; // Random distance calculation
    //         graph[i][j] = distance;
    //         graph[j][i] = distance; // Ensure distance between i and j and j and i are the same
    //     }
    // }
    // Write the graph to disk as JSON
    // let graph_file = File::create("../../../../inputs/tsp.json").expect("Failed to create graph file");
    // serde_json::to_writer(graph_file, &graph).expect("Failed to write graph to JSON");
    // println!("Graph has been written to ../../../../inputs/generated_graph.json");

    // Read the JSON file
    let file = File::open("../../../../inputs/tsp.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let graph: Vec<Vec<i32>> = serde_json::from_value(json["graph"].clone()).expect("Failed to parse graph from JSON");
    let args = Args::parse();
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&graph);

    if args.execute {
    // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let res = output.read::<i32>();
        println!("Shortest path: {}", res);
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
