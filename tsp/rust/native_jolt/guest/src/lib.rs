#![no_main]

use wasm::run_tsp;

#[jolt::provable(stack_size = 10000000)]
fn run_tsp_wrapper(graph: Vec<Vec<i32>>) -> i32 {
    run_tsp(graph)
}
