use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use wasm::bls_verify;

fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/bls_verify.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let input_value = json["numSigners"]
        .as_i64()
        .expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", input_value);
    let aggregate_signature = json["aggregateSignature"]
        .as_str()
        .expect("Failed to parse value from JSON");
    let start = std::time::Instant::now();
    bls_verify(
        input_value,
        &hex::decode(aggregate_signature).expect("Failed to decode hex string"),
    );
    let duration = start.elapsed();
    println!("Verification success");
    println!("Time elapsed: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use blst::min_pk::{PublicKey, Signature};

    // BLS Verify test using an external library, "blst"
    #[test]
    fn test_bls_verify() {
        let file =
            File::open("../../../inputs/bls_verify.json").expect("Failed to open input file");
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

        let input_value = json["numSigners"]
            .as_i64()
            .expect("Failed to parse value from JSON") as u32;
        println!("Input value: {}", input_value);
        let aggregate_signature = json["aggregateSignature"]
            .as_str()
            .expect("Failed to parse value from JSON");
        // Create a temporary JSON file for testing
        // Create the signature from the compressed bytes
        let signature = Signature::from_bytes(&hex::decode(aggregate_signature).unwrap())
            .expect("Failed to create signature");

        // Generate a random public-private key pair (for verification)
        let public_key1 = PublicKey::from_bytes(&hex::decode("85695fcbc06cc4c4c9451f4dce21cbf8de3e5a13bf48f44cdbb18e2038ba7b8bb1632d7911ef1e2e08749bddbf165352").unwrap()).expect("Failed to create public key");
        let public_key2 = PublicKey::from_bytes(&hex::decode("aefe1789d6476f60439e1168f588ea16652dc321279f05a805fbc63933e88ae9c175d6c6ab182e54af562e1a0dce41bb").unwrap()).expect("Failed to create public key");
        let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        let msg = b"message";

        // Verify the signature
        let err =
            signature.aggregate_verify(true, &[msg, msg], dst, &[&public_key1, &public_key2], true);
        assert_eq!(err, blst::BLST_ERROR::BLST_SUCCESS);
    }
}
