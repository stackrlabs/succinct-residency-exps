use hex_literal::hex;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use sha3::{Keccak256, Digest};

#[no_mangle]
/// Source: https://github.com/RustCrypto/elliptic-curves/blob/a4fadfd69be555110df1ca256dfe1023af84038a/k256/src/ecdsa.rs#L65-L92
pub fn ecdsa_verify_n(n: u32) -> u32 {
    let msg = b"example message";
    let signature = Signature::try_from(hex!(
        "46c05b6368a44b8810d79859441d819b8e7cdc8bfd371e35c53196f4bcacdb51
         35c7facce2a97b95eacba8a586d87b7958aaf8368ab29cee481f76e871dbd9cb"
    ).as_slice()).unwrap();
    let recid = RecoveryId::try_from(1u8).unwrap();
    let expected_key = VerifyingKey::from_sec1_bytes(
        &hex!("0200866db99873b09fc2fb1e3ba549b156e96d1a567e3284f5f0e859a83320cb8b")
    ).unwrap();
    for _ in 1..=n {
        let recovered_key = VerifyingKey::recover_from_digest(
            Keccak256::new_with_prefix(msg),
            &signature,
            recid
        ).unwrap();
        assert_eq!(recovered_key, expected_key);
    }
    1
}