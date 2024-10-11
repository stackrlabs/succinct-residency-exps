use risc0_zkvm::guest::env;
use wasm::keccak_n;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let keccak_start = env::cycle_count();
    let res = keccak_n(input);
    let keccak_end = env::cycle_count();
    eprintln!("keccak cycles: {}", keccak_end - keccak_start);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
