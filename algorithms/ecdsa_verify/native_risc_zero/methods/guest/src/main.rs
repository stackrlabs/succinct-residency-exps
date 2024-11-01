use risc0_zkvm::guest::env;
use k256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use hex;

fn main() {
    let total_cycles = env::cycle_count();

    let input_load_start = env::cycle_count();
    let input: u32 = env::read();
    let input_load_end = env::cycle_count();
    eprintln!("input load cycles: {}", input_load_end - input_load_start);

    let ecdsa_verify_start = env::cycle_count();
    let res = ecdsa_verify_n(input);
    let ecdsa_verify_end = env::cycle_count();
    eprintln!("ecdsa_verify cycles: {}", ecdsa_verify_end - ecdsa_verify_start);
    // write public output to the journal
    env::commit(&res);
    eprintln!("total cycles: {}", env::cycle_count() - total_cycles);
}

// NOTE: We do not use `ecdsa_verify_n` from the shared 'wasm' crate here as the `ecdsa` precompile for Risc0 is implmented differently.
fn ecdsa_verify_n(n: u32) -> bool {
    let sig_bytes: [u8; 64] = hex::decode(
        "78f44a21d6711cc8771c73c5905b8661f0cda2370a04bc5c22c8884ab550d4c43fb3e92910e2ecdec67e486ab8444a51871925f88cca33d23470fb6eaa1d16dc"
    ).expect("could not decode signature").try_into().expect("could not convert to array");
    let sig = Signature::from_bytes(&sig_bytes.into()).expect("could not parse signature");
    let verifying_key_bytes =
        hex::decode("039b8327d929a0e45285c04d19c9fffbee065c266b701972922d807228120e43f3")
            .expect("could not decode verifying key");
    let verifying_key =
        VerifyingKey::from_sec1_bytes(&verifying_key_bytes).expect("could not parse verifying key");
    let msg = b"Hello, zkVM";
    for _ in 1..=n {
        assert!(verifying_key.verify(msg, &sig).is_ok());
    }
    true
}
