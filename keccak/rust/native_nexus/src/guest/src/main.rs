#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use sha3::{Digest, Keccak256};
use hex;
use nexus_rt::read_private_input;

/// Reimplemented here and not using Wasm package because of no_std issues
fn keccak_n(n: u32) -> u32 {
    const INPUT: &[u8] = b"hello world";
    const EXPECTED_HASH: &str = "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";
    for _ in 1..=n {
        let mut hasher = Keccak256::new();
        hasher.update(INPUT);
        let result = hasher.finalize();
        assert_eq!(hex::encode(result), EXPECTED_HASH);
    }
    1
}

#[nexus_rt::profile]
fn keccak_n_exec(n: u32) -> u32 {
    keccak_n(n)
}

#[nexus_rt::main]
fn main() {
    let n = read_private_input::<u32>().expect("failed to read input");
    let result = keccak_n_exec(n);
    assert_eq!(1, result);
}