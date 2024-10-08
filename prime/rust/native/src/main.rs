use clap::Parser;
use wasm::is_prime;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/prime.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Extract the number from the JSON
    let input = json["number"]
        .as_i64()
        .expect("Failed to parse number from JSON") as i32;

    println!("Input number read from JSON: {}", input);
    let is_prime = is_prime(input);
    println!("Is prime: {}", is_prime);
}
