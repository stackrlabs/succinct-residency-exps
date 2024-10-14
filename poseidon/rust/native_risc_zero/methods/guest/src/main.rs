use risc0_zkvm::guest::env;
use wasm::poseidon_hash;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let poseidon_hash_start = env::cycle_count();
    let res = poseidon_hash(input);
    let poseidon_hash_end = env::cycle_count();
    eprintln!("poseidon_hash cycles: {}", poseidon_hash_end - poseidon_hash_start);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
