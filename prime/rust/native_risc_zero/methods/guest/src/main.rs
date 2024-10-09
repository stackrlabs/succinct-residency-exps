use risc0_zkvm::guest::env;
use wasm::is_prime;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: i32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let is_prime_start = env::cycle_count();
    let res = is_prime(input);
    let is_prime_end = env::cycle_count();
    // write public output to the journal
    eprintln!("is_prime cycles: {}", is_prime_end - is_prime_start);
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
