use risc0_zkvm::guest::env;
use wasm::bls_aggregate;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let num_signers: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let bls_aggregation_start = env::cycle_count();
    let res = bls_aggregate(num_signers);
    let bls_aggregation_end = env::cycle_count();
    // write public output to the journal
    eprintln!("bls aggregation cycles: {}", bls_aggregation_end - bls_aggregation_start);
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
