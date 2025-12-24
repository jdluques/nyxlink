use x25519_dalek::{PublicKey, StaticSecret};

use crate::{
    crypto::rng::SecureRng,
    errors::{CoreError, internal::InputError},
    memory::{lock::LockStrategy, secret::SecretVector, zeroize::ZeroizePolicy},
};

pub struct PrivateDHKey {
    secret_bytes: SecretVector,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PublicDHKey {
    key: PublicKey,
}

impl PrivateDHKey {
    pub fn generate(rng: &mut SecureRng) -> Result<Self, CoreError> {
        let secret = StaticSecret::random_from_rng(rng);
        let secret_bytes = secret.to_bytes();
        Ok(Self {
            secret_bytes: SecretVector::new(
                &secret_bytes,
                LockStrategy::BestEffort,
                ZeroizePolicy::OnDrop,
            )?,
        })
    }

    pub fn public_key(&self) -> Result<PublicDHKey, CoreError> {
        Ok(PublicDHKey {
            key: PublicKey::from(&self.inner()?),
        })
    }

    pub fn inner(&self) -> Result<StaticSecret, CoreError> {
        let secret_bytes = self.secret_bytes.as_ref();
        let secret_bytes_u32: [u8; 32] =
            secret_bytes
                .try_into()
                .map_err(|_| InputError::InvalidLength {
                    expected: 32,
                    actual: secret_bytes.len(),
                })?;
        Ok(StaticSecret::from(secret_bytes_u32))
    }
}

impl PublicDHKey {
    pub fn inner(&self) -> &PublicKey {
        &self.key
    }
}
