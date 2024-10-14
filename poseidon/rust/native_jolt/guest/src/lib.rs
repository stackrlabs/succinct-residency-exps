#![no_main]

use wasm::poseidon_hash;

#[jolt::provable(stack_size = 1000000, memory_size = 10000000, max_input_size = 10000000)]
fn poseidon_hash_wrapper(n: u32) {
    poseidon_hash(n);
}
