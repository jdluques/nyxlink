use core::{error::Error, fmt};

use crate::crypto::{algorithms::*, rng::RandomnessSource};

#[derive(Debug)]
pub(crate) enum PrimitiveError {
    InvalidKey {
        scheme: Option<SignatureScheme>,
        dh_group: Option<DHGroup>,
        context: &'static str,
    },
    InvalidNonce {
        context: &'static str,
    },
    VerificationFailed {
        scheme: SignatureScheme,
        context: &'static str,
    },
    EncryptionFailed {
        aead: AEAD,
        context: &'static str,
    },
    DecryptionFailed {
        aead: AEAD,
        context: &'static str,
    },
    DerivationFailed {
        kdf: KDF,
        context: &'static str,
    },
    RandomnessUnavailable {
        source: RandomnessSource,
    },
    ZeroizationFailed {
        context: &'static str,
    },
}

impl fmt::Display for PrimitiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrimitiveError::*;

        match self {
            InvalidKey {
                scheme,
                dh_group,
                context,
            } => {
                write!(
                    f,
                    "Invalid key (scheme={:?}, dh_group={:?}): {}",
                    scheme, dh_group, context
                )
            }

            InvalidNonce { context } => write!(f, "Invalid nonce: {}", context),

            VerificationFailed { scheme, context } => {
                write!(f, "Verification failed using {:?}: {}", scheme, context)
            }

            EncryptionFailed { aead, context } => {
                write!(f, "Encryption failed using {:?}: {}", aead, context)
            }

            DecryptionFailed { aead, context } => {
                write!(f, "Decryption failed using {:?}:{}", aead, context)
            }

            DerivationFailed { kdf, context } => {
                write!(f, "Key derivation failed using {:?}: {}", kdf, context)
            }

            RandomnessUnavailable { source } => {
                write!(f, "Randomness unavailable from {:?}", source)
            }

            ZeroizationFailed { context } => {
                write!(f, "Zeroization failed: {}", context)
            }
        }
    }
}

impl Error for PrimitiveError {}
