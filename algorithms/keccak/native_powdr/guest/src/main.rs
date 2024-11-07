use powdr_riscv_runtime;
use powdr_riscv_runtime::io::{read, write};
use wasm::keccak_n;

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let input: u64 = read(1);
    let res = keccak_n(input.try_into().unwrap());
    assert_eq!(res, 1);
}
