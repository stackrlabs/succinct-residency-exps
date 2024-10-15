use risc0_zkvm::guest::env;
use wasm::{bls_verify, hash, PublicKey, Signature, Serialize};

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let aggregate_signature: Vec<u8> = env::read();
    let public_keys: Vec<Vec<u8>> = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let decode_start = env::cycle_count();
    let public_keys = public_keys
        .iter()
        .map(|pk| PublicKey::from_bytes(pk).expect("failed to decode public key"))
        .collect::<Vec<_>>();
    let aggregated_signature =
        Signature::from_bytes(&aggregate_signature).expect("failed to decode aggregated signature");
    let message = "message".as_bytes().to_vec();
    let hash = hash(&message);
    let decode_end = env::cycle_count();
    eprintln!("deserialize cycles: {}", decode_end - decode_start);

    let bls_verify_start = env::cycle_count();
    let res = bls_verify(aggregated_signature, public_keys, hash);
    let bls_verify_end = env::cycle_count();
    // write public output to the journal
    eprintln!("bls verify cycles: {}", bls_verify_end - bls_verify_start);
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}
