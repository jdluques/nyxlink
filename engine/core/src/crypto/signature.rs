use ed25519_dalek::ed25519::signature::Signer;

use crate::{
    crypto::keys::signing::{PrivateSigningKey, PublicSigningKey},
    errors::{
        CoreError,
        internal::{InputError, PrimitiveError},
    },
};

const SIGNING_DOMAIN: &[u8] = b"com.nyxlink.nyxlink-engine-core/ed25519/signature/v1";

pub struct Signature {
    bytes: [u8; 64],
}

impl Signature {
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, InputError> {
        Ok(Self {
            bytes: bytes.try_into().map_err(|_| InputError::InvalidLength {
                expected: 64,
                actual: bytes.len(),
            })?,
        })
    }

    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.bytes
    }
}

pub fn sign(key: &PrivateSigningKey, message: &[u8]) -> Result<Signature, CoreError> {
    let signature: ed25519_dalek::Signature =
        key.inner()?.sign(&[SIGNING_DOMAIN, message].concat());
    Ok(Signature {
        bytes: signature.to_bytes(),
    })
}

pub fn verify(
    key: &PublicSigningKey,
    message: &[u8],
    signature: &Signature,
) -> Result<(), PrimitiveError> {
    let signature = ed25519_dalek::Signature::from_bytes(&signature.bytes);
    Ok(key
        .inner()
        .verify_strict(&[SIGNING_DOMAIN, message].concat(), &signature)?)
}
