use ed25519_dalek::ed25519::signature::Signer;

use crate::{
    crypto::{
        algorithms,
        hash::hash_with_domain,
        keys::signing::{PrivateSigningKey, PublicSigningKey},
    },
    errors::{CoreError, internal::PrimitiveError},
};

const SIGNING_DOMAIN: &[u8] = b"com.nyxlink.nyxlink-engine-core/ed25519/signature/v1";

#[derive(Debug, Clone)]
pub struct Signature {
    bytes: [u8; 64],
}

impl Signature {
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.bytes
    }
}

pub fn sign(key: &PrivateSigningKey, message: &[u8]) -> Result<Signature, CoreError> {
    let digest = hash_with_domain(SIGNING_DOMAIN, message, algorithms::Hash::SHA512);
    let signature = key.inner()?.sign(&digest);
    Ok(Signature {
        bytes: signature.to_bytes(),
    })
}

pub fn verify(
    key: &PublicSigningKey,
    message: &[u8],
    signature: &Signature,
) -> Result<(), PrimitiveError> {
    let digest = hash_with_domain(SIGNING_DOMAIN, message, algorithms::Hash::SHA512);
    let signature = ed25519_dalek::Signature::from_bytes(&signature.bytes);
    Ok(key.inner().verify_strict(&digest, &signature)?)
}
