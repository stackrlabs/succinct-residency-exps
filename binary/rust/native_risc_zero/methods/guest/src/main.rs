use risc0_zkvm::guest::env;
use wasm::binary_search_impl;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: i32 = env::read();
    let list: Vec<i32> = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let search_start = env::cycle_count();
    let found = binary_search_impl(list, input);
    let search_end = env::cycle_count();
    // write public output to the journal
    eprintln!("search cycles: {}", search_end - search_start);
    env::commit(&found);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
