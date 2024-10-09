#![no_main]

use wasm::is_prime;

#[jolt::provable]
fn is_prime_wrapper(n: i32) -> bool {
    is_prime(n)
}
