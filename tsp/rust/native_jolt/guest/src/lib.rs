#![no_main]

use wasm::run_tsp;

#[jolt::provable]
fn run_tsp_wrapper(graph: Vec<Vec<i32>>) -> i32 {
    run_tsp(graph)
}
