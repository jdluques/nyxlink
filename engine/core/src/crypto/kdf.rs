use hkdf::Hkdf;

use crate::{
    crypto::algorithms,
    errors::{
        CoreError,
        internal::{InputError, PrimitiveError},
    },
    memory::secret::SecretVector,
};

pub enum DerivedKeyKind {
    RootChainingKey,
    MessageChainingKey,
    MessageEncryptionKey,
    MessageNonceKey,
    HeaderEncryptionKey,
    AttachmentEncryptionKey,
}

impl DerivedKeyKind {
    pub(self) fn key_buf(&self) -> Vec<u8> {
        match self {
            Self::RootChainingKey | Self::MessageChainingKey => vec![0; 64],
            Self::MessageEncryptionKey
            | Self::MessageNonceKey
            | Self::HeaderEncryptionKey
            | Self::AttachmentEncryptionKey => vec![0; 32],
        }
    }

    pub(self) const fn context(self) -> &'static [u8] {
        match self {
            Self::RootChainingKey => b"com.nyxlink.nyxlink-core-engine/v1/hkdf/root-chaining-key",
            Self::MessageChainingKey => {
                b"com.nyxlink.nyxlink-core-engine/v1/hkdf/message-chaining-key"
            }
            Self::MessageEncryptionKey => {
                b"com.nyxlink.nyxlink-core-engine/v1/hkdf/message-encryption-key"
            }
            Self::MessageNonceKey => b"com.nyxlink.nyxlink-core-engine/v1/hkdf/message-nonce-key",
            Self::HeaderEncryptionKey => {
                b"com.nyxlink.nyxlink-core-engine/v1/hkdf/header-encryption-key"
            }
            Self::AttachmentEncryptionKey => {
                b"com.nyxlink.nyxlink-core-engine/v1/hkdf/attachment-encryption-key"
            }
        }
    }
}

pub fn hkdf(
    alg: algorithms::KDF,
    salt: &[u8],
    ikm: &[u8],
    key_kind: DerivedKeyKind,
) -> Result<SecretVector, CoreError> {
    if salt.is_empty() {
        return Err(InputError::InvalidValue {
            context: "Salt must not be empty",
        })?;
    }

    if ikm.is_empty() {
        return Err(InputError::InvalidValue {
            context: "Input Key Material must not be empty",
        })?;
    }

    let mut buf = key_kind.key_buf();
    let info = key_kind.context();

    match alg {
        algorithms::KDF::HKDF_SHA512 => {
            use sha2::Sha512;
            let hk = Hkdf::<Sha512>::new(Some(salt), ikm);
            hk.expand(info, &mut buf)
                .map_err(|_| PrimitiveError::DerivationFailed {
                    kdf: algorithms::KDF::HKDF_SHA512,
                })?;

            Ok(SecretVector::new(
                &buf,
                crate::memory::lock::LockStrategy::BestEffort,
                crate::memory::zeroize::ZeroizePolicy::OnDrop,
            )?)
        }
    }
}
