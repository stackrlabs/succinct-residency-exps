#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use wasm::merkelize;

#[jolt::provable(stack_size = 100000000000, memory_size = 100000000000)]
fn merkelize_wrapper(leaves_count: i32) -> u32 {
    merkelize(leaves_count)
}
