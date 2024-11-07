use powdr_riscv_runtime;
use powdr_riscv_runtime::io::{read, write};
use wasm::nth_prime;

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let input: u64 = read(1);
    // Read the claimed sum from channel 2.
    let expected: u64 = read(2);
    let res = nth_prime(input);
    assert_eq!(res, expected);
}
