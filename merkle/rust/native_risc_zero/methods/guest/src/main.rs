use risc0_zkvm::guest::env;
use wasm::merkelize_impl;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let leaves: Vec<Vec<u8>> = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let merkelization_start = env::cycle_count();
    let root = merkelize_impl(leaves);
    let merkelization_end = env::cycle_count();
    // write public output to the journal
    eprintln!("merkelization cycles: {}", merkelization_end - merkelization_start);
    env::commit(&root);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
