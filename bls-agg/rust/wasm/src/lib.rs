use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve, HashToField},
    G2Affine,G2Projective, Scalar,
};
use sha2::{digest::generic_array::typenum::U48, digest::generic_array::GenericArray, Sha256};
use hkdf::Hkdf;
use thiserror::Error;

#[no_mangle]
pub fn bls_aggregate(num_signers: u32) -> u32 {
    let private_keys: Vec<_> = (0..num_signers)
            .map(|i| PrivateKey::new(&[i as u8; 32]))
            .collect();

        // generate messages
        let messages: Vec<Vec<u8>> = (0..num_signers)
            .map(|_| "message".as_bytes().to_vec())
            .collect();

        // sign messages
        let sigs = messages
            .iter()
            .zip(&private_keys)
            .map(|(message, pk)| pk.sign(message))
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
pub fn aggregate(signatures: &[Signature]) -> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
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




#[derive(Debug, Error)]
pub enum Error {
    #[error("Size mismatch")]
    SizeMismatch,
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Group decode error")]
    GroupDecode,
    #[error("Curve decode error")]
    CurveDecode,
    #[error("Prime field decode error")]
    FieldDecode,
    #[error("Invalid Private Key")]
    InvalidPrivateKey,
    #[error("Zero sized input")]
    ZeroSizedInput,
}