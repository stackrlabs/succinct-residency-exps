#![no_main]

use wasm::binary_search_impl;

#[jolt::provable(max_input_size = 1500000)]
fn binary_search_wrapper(search_list: Vec<i32>, search_value: i32) -> i32 {
    binary_search_impl(search_list, search_value)
}
