use blst::min_pk::{PublicKey, SecretKey, Signature};
use blst::*;
use rand::thread_rng;
use rand::Rng;
use std::convert::TryInto;

fn main() {
    // Example from your input
    let x_real = "00abc29d88c196991745975c9206b8f56ff120b70df72ce36024da3382e398b47189cf1e9e6e998212e8ba5ab2dc79e2";
    let x_imaginary = "0056bb0d3fb4f892290380e62d0603e0af6bd5db8e4f405876ff397e473ace7310b3ff8509aeffb47da06e0e298d6327";
    let y_real = "07512e3a97c6de7b86fd6b3be4b2a1a6732f34cdd4838d309f45f4a3dbca35ccf1821ef6814c20389c870fb4acac4937";
    let y_imaginary = "11e4f26ddead8f98686b80580c8653bdc1ecac03f505159387c6cf1feaeb6be625c9c1a850ab134d3e2b770a8a9ee705";

    // Convert hex to bytes
    let x_real_bytes = hex::decode(x_real).expect("Invalid hex for x_real");
    let x_imaginary_bytes = hex::decode(x_imaginary).expect("Invalid hex for x_imaginary");
    let y_real_bytes = hex::decode(y_real).expect("Invalid hex for y_real");
    let y_imaginary_bytes = hex::decode(y_imaginary).expect("Invalid hex for y_imaginary");

    // Helper function to convert 48-byte arrays to [6; u64] arrays for blst_fp
    fn bytes_to_fp(bytes: &[u8]) -> [u64; 6] {
        let mut arr = [0u64; 6];
        for (i, chunk) in bytes.chunks(8).enumerate() {
            arr[i] = u64::from_be_bytes(chunk.try_into().expect("slice with incorrect length"));
        }
        arr
    }

    // Convert bytes to blst_fp
    let x_real_fp = bytes_to_fp(&x_real_bytes);
    let x_imaginary_fp = bytes_to_fp(&x_imaginary_bytes);
    let y_real_fp = bytes_to_fp(&y_real_bytes);
    let y_imaginary_fp = bytes_to_fp(&y_imaginary_bytes);

    // Create blst_fp2 elements for x and y
    let mut x_fp2 = blst_fp2::default();
    let mut y_fp2 = blst_fp2::default();

    unsafe {
        blst_fp_from_uint64(&mut x_fp2.fp[0], &x_real_fp[0]);
        blst_fp_from_uint64(&mut x_fp2.fp[1], &x_imaginary_fp[0]);
        blst_fp_from_uint64(&mut y_fp2.fp[0], &y_real_fp[0]);
        blst_fp_from_uint64(&mut y_fp2.fp[1], &y_imaginary_fp[0]);
    }

    // Create a G2Affine point for the signature
    let mut signature_affine = blst_p2_affine {
        x: x_fp2,
        y: y_fp2,
    };

    // Create a compressed representation of the signature point
    let mut compressed_sig = [0u8; 96];
    unsafe {
        blst_p2_affine_compress(compressed_sig.as_mut_ptr(), &signature_affine);
    }

    // Create the signature from the compressed bytes
    let signature = Signature::from_bytes(&compressed_sig).expect("Failed to create signature");

    // Generate a random public-private key pair (for verification)
    let pvt_key = SecretKey::from_bytes(&hex::decode("045690f6a8fb6fac9ce7c1171740e4e2e1f572036240e6a7a091c0dae33b354a").unwrap()).unwrap();

    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    let msg = b"message";

    // Verify the signature
    let err = signature.verify(true, msg, dst, &[], &pvt_key.sk_to_pk(), false);
    assert_eq!(err, blst::BLST_ERROR::BLST_SUCCESS);
    if err == blst::BLST_ERROR::BLST_SUCCESS {
        println!("Signature is valid!");
    } else {
        println!("Signature is invalid!");
    }
}
