#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;

use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve, HashToField},
    Bls12, G1Affine, G1Projective, G2Affine, G2Projective, Gt, MillerLoopResult, Scalar,
};
use hkdf::Hkdf;
use nexus_rt::println;
use alloc::vec::Vec;
use nexus_rt::read_private_input;
use group::Curve;
use pairing::MultiMillerLoop;
use sha2::{digest::generic_array::typenum::U48, digest::generic_array::GenericArray, Sha256};
use hex;
use alloc::vec;

#[nexus_rt::profile]
pub fn bls_verify(aggregated_signature: Signature, public_keys: Vec<PublicKey>, msg_hash: G2Projective) -> u32 {
    assert!(
        verify(&aggregated_signature, &vec![msg_hash; public_keys.len() as usize], &public_keys),
            "failed to verify"
        );
    1
}

#[nexus_rt::main]
fn main() {
    // let input_value = read_private_input::<u32>().expect("failed to read input");
    let private_keys: Vec<PrivateKey> = (0..2)
        .map(|i| PrivateKey::new(&[i as u8; 32]))
        .collect();    // let n = read_private_input::<u32>().expect("failed to read input");

    let public_keys = private_keys
        .iter()
        .map(|pk| pk.public_key().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let public_keys = public_keys
        .iter()
        .map(|pk| PublicKey::from_bytes(pk).expect("failed to decode public key"))
        .collect::<Vec<_>>();
    let aggregated_signature =
        Signature::from_bytes(&hex::decode("a056bb0d3fb4f892290380e62d0603e0af6bd5db8e4f405876ff397e473ace7310b3ff8509aeffb47da06e0e298d632700abc29d88c196991745975c9206b8f56ff120b70df72ce36024da3382e398b47189cf1e9e6e998212e8ba5ab2dc79e2").unwrap()).expect("failed to decode aggregated signature");
    let message = "message".as_bytes().to_vec();
    let hash = hash(&message);
    bls_verify(aggregated_signature, public_keys, hash);
}

#[derive(Debug)]
pub struct Signature(G2Affine);

impl From<G2Projective> for Signature {
    fn from(val: G2Projective) -> Self {
        Signature(val.into())
    }
}
impl From<Signature> for G2Projective {
    fn from(val: Signature) -> Self {
        val.0.into()
    }
}

impl From<G2Affine> for Signature {
    fn from(val: G2Affine) -> Self {
        Signature(val)
    }
}

impl From<Signature> for G2Affine {
    fn from(val: Signature) -> Self {
        val.0
    }
}

const G2_COMPRESSED_SIZE: usize = 96;

impl Serialize for Signature {
    fn write_bytes(&self) -> Vec<u8> {
        self.0.to_compressed().to_vec()
    }

    fn from_bytes(raw: &[u8]) -> Result<Self, ()> {
        let g2 = g2_from_slice(raw)?;
        Ok(g2.into())
    }
}

fn g2_from_slice(raw: &[u8]) -> Result<G2Affine, ()> {
    if raw.len() != G2_COMPRESSED_SIZE {
        return Err(());
    }

    let mut res = [0u8; G2_COMPRESSED_SIZE];
    res.copy_from_slice(raw);

    Option::from(G2Affine::from_compressed(&res)).ok_or(())
}


/// Aggregate signatures by multiplying them together.
/// Calculated by `signature = \sum_{i = 0}^n signature_i`.
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

     /// Get the public key for this private key.
    /// Calculated by `pk = g1 * sk`.
    pub fn public_key(&self) -> PublicKey {
        let mut pk = G1Projective::generator();
        pk *= self.0;

        PublicKey(pk)
    }
}

const CSUITE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
pub(crate) const G1_COMPRESSED_SIZE: usize = 48;


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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PublicKey(pub(crate) G1Projective);

impl PublicKey {
    pub fn as_affine(&self) -> G1Affine {
        self.0.to_affine()
    }

    pub fn verify<T: AsRef<[u8]>>(&self, sig: Signature, message: T) -> bool {
        verify_messages(&sig, &[message.as_ref()], &[*self])
    }
}

pub trait Serialize: Sized {
    /// Writes the key to the given writer.
    fn write_bytes(&self) -> Vec<u8>;

    /// Recreate the key from bytes in the same form as `write_bytes` produced.
    fn from_bytes(raw: &[u8]) -> Result<Self, ()>;

    fn as_bytes(&self) -> Vec<u8> {
        self.write_bytes()
    }
}

impl Serialize for PublicKey {
    fn write_bytes(&self) -> Vec<u8> {
        let t = self.0.to_affine();
        let tmp = t.to_compressed();
        tmp.as_ref().to_vec()
    }

    fn from_bytes(raw: &[u8]) -> Result<Self, ()> {
        if raw.len() != G1_COMPRESSED_SIZE {
            return Err(());
        }

        let mut res = [0u8; G1_COMPRESSED_SIZE];
        res.as_mut().copy_from_slice(raw);
        let affine: G1Affine =
            Option::from(G1Affine::from_compressed(&res)).ok_or(())?;

        Ok(PublicKey(affine.into()))
    }
}


/// Verifies that the signature is the actual aggregated signature of messages - pubkeys.
/// Calculated by `e(g1, signature) == \prod_{i = 0}^n e(pk_i, hash_i)`.
pub fn verify_messages(
    signature: &Signature,
    messages: &[&[u8]],
    public_keys: &[PublicKey],
) -> bool {
    let hashes: Vec<_> = messages.iter().map(|msg| hash(msg)).collect();

    verify(signature, &hashes, public_keys)
}

/// Verifies that the signature is the actual aggregated signature of hashes - pubkeys.
/// Calculated by `e(g1, signature) == \prod_{i = 0}^n e(pk_i, hash_i)`.
pub fn verify(signature: &Signature, hashes: &[G2Projective], public_keys: &[PublicKey]) -> bool {
    if hashes.is_empty() || public_keys.is_empty() {
        return false;
    }

    let n_hashes = hashes.len();

    if n_hashes != public_keys.len() {
        return false;
    }

    // zero keys should always fail.
    if public_keys.iter().any(|pk| pk.0.is_identity().into()) {
        return false;
    }

    // Enforce that messages are distinct as a countermeasure against BLS's rogue-key attack.
    // See Section 3.1. of the IRTF's BLS signatures spec:
    // https://tools.ietf.org/html/draft-irtf-cfrg-bls-signature-02#section-3.1
    // NOTE: Skipping this check to allow duplicate messages
    // if hashes
    //     .iter()
    //     // This is the best way to get something we can actually hash.
    //     .map(|h| G2Affine::from(h).to_compressed())
    //     .collect::<HashSet<_>>()
    //     .len()
    //     != hashes.len()
    // {
    //     return false;
    // }

    let mut ml = public_keys
        .iter()
        .zip(hashes.iter())
        .map(|(pk, h)| {
            let pk = pk.as_affine();
            let h = G2Affine::from(h).into();
            Bls12::multi_miller_loop(&[(&pk, &h)])
        })
        .fold(MillerLoopResult::default(), |acc, cur| acc + cur);

    let g1_neg = -G1Affine::generator();

    ml += Bls12::multi_miller_loop(&[(&g1_neg, &signature.0.into())]);

    ml.final_exponentiation() == Gt::identity()
}

