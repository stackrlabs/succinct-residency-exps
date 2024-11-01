use powdr_riscv_runtime;
use powdr_riscv_runtime::io::{read, write};
use wasm::{bls_verify, hash, PublicKey, Serialize, Signature};

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let aggregate_signature: Vec<u8> = read(1);
    let public_keys: Vec<Vec<u8>> = read(2);
    let public_keys = public_keys
        .iter()
        .map(|pk| PublicKey::from_bytes(pk).expect("failed to decode public key"))
        .collect::<Vec<_>>();
    let aggregated_signature =
        Signature::from_bytes(&aggregate_signature).expect("failed to decode aggregated signature");
    let message = "message".as_bytes().to_vec();
    let hash = hash(&message);
    let res = bls_verify(aggregated_signature, public_keys, hash);
    assert_eq!(res, 1);
}
