use risc0_zkvm::guest::env;
use wasm::run_tsp;

fn main() {
    let total_cycles = env::cycle_count();

    // read the input
    let input_load_start = env::cycle_count();
    let graph = env::read();
    eprintln!("input load cycles: {}", env::cycle_count() - input_load_start);

    // run the TSP algorithm
    let tsp_start = env::cycle_count();
    let result = run_tsp(graph);
    eprintln!("TSP result: {}", result);
    eprintln!("TSP cycles: {}", env::cycle_count() - tsp_start);

    // write public output to the journal
    env::commit(&result);
    eprintln!("Total cycles: {}", env::cycle_count() - total_cycles);
}
