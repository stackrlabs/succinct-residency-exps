#![no_main]

use wasm::merkelize_impl;

#[jolt::provable]
fn merkelize_wrapper(leaves: Vec<Vec<u8>>) -> [u8; 32] {
    merkelize_impl(leaves)
}
