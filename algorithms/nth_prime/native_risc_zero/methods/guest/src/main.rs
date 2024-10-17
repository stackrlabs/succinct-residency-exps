use risc0_zkvm::guest::env;
use wasm::nth_prime;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: u64 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let nth_prime_start = env::cycle_count();
    let res = nth_prime(input);
    let nth_prime_end = env::cycle_count();
    eprintln!("nth_prime cycles: {}", nth_prime_end - nth_prime_start);
    println!("The {}th prime is {}.", input, res);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
