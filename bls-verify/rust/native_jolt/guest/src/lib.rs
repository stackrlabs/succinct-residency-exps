#![no_main]

use wasm::bls_verify;

#[jolt::provable(stack_size = 1_000_000, memory_size = 1_000_000)]
fn bls_verify_wrapper(num_signers: u32, aggregate_signature: Vec<u8>) -> u32 {
    bls_verify(num_signers, &aggregate_signature)
}

