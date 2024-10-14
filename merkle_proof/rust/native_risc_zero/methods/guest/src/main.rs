use risc0_zkvm::guest::env;
use wasm::generate_merkle_proof;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let merkle_proof_start = env::cycle_count();
    let res = generate_merkle_proof(input);
    let merkle_proof_end = env::cycle_count();
    eprintln!("merkle_proof cycles: {}", merkle_proof_end - merkle_proof_start);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
