#![no_main]

use wasm::poseidon_hash;

#[jolt::provable(stack_size = 10000000, memory_size = 100000000)]
fn poseidon_hash_wrapper(n: u32) {
    poseidon_hash(n);
}
