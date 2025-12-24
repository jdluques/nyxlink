use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::{
    crypto::rng::SecureRng,
    errors::{CoreError, internal::InputError},
    memory::{lock::LockStrategy, secret::SecretVector, zeroize::ZeroizePolicy},
};

pub struct PrivateSigningKey {
    key_bytes: SecretVector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicSigningKey {
    key: VerifyingKey,
}

impl PrivateSigningKey {
    pub fn generate(rng: &mut SecureRng) -> Result<Self, CoreError> {
        let key = SigningKey::generate(rng);
        let key_bytes = key.to_bytes();
        Ok(Self {
            key_bytes: SecretVector::new(
                &key_bytes,
                LockStrategy::BestEffort,
                ZeroizePolicy::OnDrop,
            )?,
        })
    }

    pub fn public_key(&self) -> Result<PublicSigningKey, CoreError> {
        Ok(PublicSigningKey {
            key: self.inner()?.verifying_key(),
        })
    }

    pub(crate) fn inner(&self) -> Result<SigningKey, CoreError> {
        let key_bytes = self.key_bytes.as_ref();
        Ok(SigningKey::from_bytes(key_bytes.try_into().map_err(
            |_| InputError::InvalidLength {
                expected: 32,
                actual: key_bytes.len(),
            },
        )?))
    }
}

impl PublicSigningKey {
    pub(crate) fn inner(&self) -> &VerifyingKey {
        &self.key
    }
}
