use hex_literal::hex;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

/// Code adapted from: https://github.com/RustCrypto/elliptic-curves/blob/a4fadfd69be555110df1ca256dfe1023af84038a/k256/src/ecdsa.rs#L65-L92
/// Parameters adapted from: https://github.com/succinctlabs/sp1/blob/6512b56296c2c5e53b10cce1a741173a3d2dde68/examples/patch-testing/program/src/main.rs#L126-L138
#[no_mangle]
pub fn ecdsa_verify_n(n: u32) -> u32 {
    // message is "Some data"
    let msg_hash = hex!("43a26051362b8040b289abe93334a5e3662751aa691185ae9e9a2e1e0c169350");
    let signature = Signature::try_from(
        hex!("46c05b6368a44b8810d79859441d819b8e7cdc8bfd371e35c53196f4bcacdb5135c7facce2a97b95eacba8a586d87b7958aaf8368ab29cee481f76e871dbd9cb").as_slice()
    )
    .expect("failed to parse signature");
    let recid = RecoveryId::try_from(1u8).unwrap();
    let expected_key = VerifyingKey::from_sec1_bytes(
        &hex!("0473cb89a27b4699eeb65939110d1209c8e02915bdbfc2befc4e1ea5b9fbb69e8800c3b90474195b1a603cbeb56f58717e17389ed51edb7e1838435e4d6c384fd8")
    ).expect("failed to parse verifying key");
    for _ in 1..=n {
        let recovered_key = VerifyingKey::recover_from_prehash(&msg_hash, &signature, recid)
            .expect("failed to recover key");
        assert_eq!(&recovered_key, &expected_key);
    }
    1
}
