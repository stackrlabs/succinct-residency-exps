#![no_std]

use alloy_primitives::{address, Signature};
use ark_std::str::FromStr;
use ark_std::println;

/// Source: https://github.com/succinctlabs/sp1/blob/6512b56296c2c5e53b10cce1a741173a3d2dde68/examples/patch-testing/program/src/main.rs#L126-L138
#[no_mangle]
pub fn ecdsa_verify_n(n: u32) -> u32 {
    let sig = Signature::from_str(
        "b91467e570a6466aa9e9876cbcd013baba02900b8979d43fe208a4a4f339f5fd6007e74cd82e037b800186422fc2da167c747ef045e5d18a5f5d4300f8e1a0291c"
    ).expect("could not parse signature");
    let expected_address = address!("2c7536E3605D9C16a7a3D7b1898e529396a65c23");
    for _ in 1..=n {
        println!("cycle-tracker-start: k256 verify");
        let recovered_address =
            sig.recover_address_from_msg("Some data").expect("could not recover address");
        println!("cycle-tracker-end: k256 verify");
        assert_eq!(recovered_address, expected_address);
    }
    1
}