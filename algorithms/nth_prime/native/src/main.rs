use wasm::nth_prime;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/nth_prime.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Extract the number from the JSON
    let input = json["n"]
        .as_u64()
        .expect("Failed to parse n from JSON");
    println!("Input n read from JSON: {}", input);

    let start = std::time::Instant::now();
    let nth_prime_res = nth_prime(input);
    let duration = start.elapsed();
    
    println!("The {}th prime is {}.", input, nth_prime_res);
    println!("Time elapsed: {:?}", duration);
}
