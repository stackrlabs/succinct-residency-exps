extern crate alloc;
use alloc::str::FromStr;
use starknet_crypto::poseidon_hash_single;
use starknet_types_core::felt::Felt;

/// Ref: https://github.com/xJonathanLEI/starknet-rs/blob/master/starknet-crypto/benches/poseidon_hash.rs
#[no_mangle]
pub fn poseidon_hash(n: u32) -> u32 {
    let felt = Felt::from_str("1").unwrap();
    let expected_hash = "0x06d226d4c804cd74567f5ac59c6a4af1fe2a6eced19fb7560a9124579877da25";
    for _ in 1..=n {
        let hash = poseidon_hash_single(felt).to_fixed_hex_string();
        assert_eq!(hash, expected_hash);
    }
    1
}
