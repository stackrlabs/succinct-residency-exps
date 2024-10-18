#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;

use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve, HashToField},
    G2Affine, G2Projective, Scalar,
};
use hkdf::Hkdf;
use nexus_rt::println;
use sha2::{digest::generic_array::typenum::U48, digest::generic_array::GenericArray, Sha256};
use alloc::vec::Vec;

pub fn bls_aggregate(num_signers: u32) -> u32 {
    let private_keys: Vec<_> = (0..num_signers)
        .map(|i| PrivateKey::new(&[i as u8; 32]))
        .collect();

    let message = "message".as_bytes().to_vec();
    // sign messages
    let sigs = private_keys
        .iter()
        .map(|pk| pk.sign(&message))
        .collect::<Vec<Signature>>();

    let aggregated_signature = aggregate(&sigs).expect("failed to aggregate");
    println!("aggregated_signature: {:?}", aggregated_signature);
    1
}

#[derive(Debug)]
pub struct Signature(G2Affine);

impl From<G2Projective> for Signature {
    fn from(val: G2Projective) -> Self {
        Signature(val.into())
    }
}

/// Aggregate signatures by multiplying them together.
/// Calculated by `signature = \sum_{i = 0}^n signature_i`.
#[nexus_rt::profile]
pub fn aggregate(signatures: &[Signature]) -> Result<Signature, ()> {
    if signatures.is_empty() {
        return Err(());
    }

    let res = signatures
        .iter()
        .fold(G2Projective::identity(), |acc, signature| acc + signature.0);

    Ok(Signature(res.into()))
}

pub struct PrivateKey(pub(crate) Scalar);
impl PrivateKey {
    /// Generate a deterministic private key from the given bytes.
    ///
    /// They must be at least 32 bytes long to be secure, will panic otherwise.
    pub fn new<T: AsRef<[u8]>>(msg: T) -> Self {
        PrivateKey(key_gen(msg))
    }

    /// Sign the given message.
    /// Calculated by `signature = hash_into_g2(message) * sk`
    pub fn sign<T: AsRef<[u8]>>(&self, message: T) -> Signature {
        let mut p = hash(message.as_ref());
        p *= self.0;

        p.into()
    }
}

const CSUITE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

pub fn hash(msg: &[u8]) -> G2Projective {
    <G2Projective as HashToCurve<ExpandMsgXmd<sha2::Sha256>>>::hash_to_curve(msg, CSUITE)
}

fn key_gen<T: AsRef<[u8]>>(data: T) -> Scalar {
    // "BLS-SIG-KEYGEN-SALT-"
    const SALT: &[u8] = b"BLS-SIG-KEYGEN-SALT-";

    let data = data.as_ref();
    assert!(data.len() >= 32, "IKM must be at least 32 bytes");

    // HKDF-Extract
    let mut msg = data.as_ref().to_vec();
    // append zero byte
    msg.push(0);
    let prk = Hkdf::<Sha256>::new(Some(SALT), &msg);

    // HKDF-Expand
    // `result` has enough length to hold the output from HKDF expansion
    let mut result = GenericArray::<u8, U48>::default();
    assert!(prk.expand(&[0, 48], &mut result).is_ok());

    Scalar::from_okm(&result)
}

#[nexus_rt::main]
fn main() {
    // let n = read_private_input::<u32>().expect("failed to read input");
    bls_aggregate(2);
}
