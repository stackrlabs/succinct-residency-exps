#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;
use alloc::str::FromStr;
use starknet_crypto::poseidon_hash_single;
use starknet_types_core::felt::Felt;
use nexus_rt::read_private_input;

// Reimplemented here and not using Wasm package because of no_std
fn poseidon_hash(n: u32) -> u32 {
    let felt = Felt::from_str("1").unwrap();
    let expected_hash = "0x06d226d4c804cd74567f5ac59c6a4af1fe2a6eced19fb7560a9124579877da25";
    for _ in 1..=n {
        let hash = poseidon_hash_single(felt).to_fixed_hex_string();
        assert_eq!(hash, expected_hash);
    }
    1
}

#[nexus_rt::profile]
fn poseidon_hash_exec(n: u32) -> u32 {
    poseidon_hash(n)
}

#[nexus_rt::main]
fn main() {
    let n = read_private_input::<u32>().expect("failed to read input");
    let result = poseidon_hash_exec(n);
    assert_eq!(1, result);
}
