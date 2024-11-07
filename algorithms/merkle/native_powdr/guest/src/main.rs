use powdr_riscv_runtime;
use powdr_riscv_runtime::io::{read, write};
use wasm::merkelize;

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let num_leaves_base2: i32 = read(1);
    let res = merkelize(num_leaves_base2);
    assert_eq!(res, 1);
}
