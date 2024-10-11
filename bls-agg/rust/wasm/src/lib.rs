use blst::min_pk::{AggregateSignature, SecretKey, Signature};

#[no_mangle]
pub fn bls_aggregate(num_signers: u32) -> u32 {
    let mut private_key_bytes = Vec::new();
    for i in 0..num_signers {
        let sk_bytes = [i as u8; 32];
        private_key_bytes.push(sk_bytes);
    }

    let mut signatures = Vec::new();
    let message = b"42";

    // Generate public keys and sign the message using hardcoded private keys
    for sk_bytes in private_key_bytes.iter() {
        let sk = SecretKey::key_gen(sk_bytes, &[]).unwrap();

        // Sign the message
        let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        let sig = sk.sign(message, dst, &[]);

        // Accumulate the signatures
        signatures.push(sig);
    }

    // Aggregate the signatures
    let aggregated_signature = AggregateSignature::aggregate(&signatures.iter().collect::<Vec<&Signature>>(), true)
        .expect("Failed to aggregate signatures");

    println!("Aggregated signature: {:?}", aggregated_signature);

    num_signers
}
