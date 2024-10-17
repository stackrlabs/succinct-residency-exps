use risc0_zkvm::guest::env;
use wasm::merkelize;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let leaves_count: i32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let merkelization_start = env::cycle_count();
    let res = merkelize(leaves_count);
    let merkelization_end = env::cycle_count();
    // write public output to the journal
    eprintln!("merklization cycles: {}", merkelization_end - merkelization_start);
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
