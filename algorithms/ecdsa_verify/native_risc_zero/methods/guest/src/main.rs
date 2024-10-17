use risc0_zkvm::guest::env;
use wasm::ecdsa_verify_n;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let ecdsa_verify_start = env::cycle_count();
    let res = ecdsa_verify_n(input);
    let ecdsa_verify_end = env::cycle_count();
    eprintln!("ecdsa_verify cycles: {}", ecdsa_verify_end - ecdsa_verify_start);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
