use core::{error::Error, fmt};

use crate::crypto::{
    algorithms::{self, *},
    rng::RandomnessSource,
};

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
    },
    EncryptionFailed {
        aead: AEAD,
    },
    DecryptionFailed {
        aead: AEAD,
    },
    DerivationFailed {
        kdf: KDF,
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

            VerificationFailed { scheme } => {
                write!(f, "Verification failed using {:?}", scheme)
            }

            EncryptionFailed { aead } => {
                write!(f, "Encryption failed using {:?}", aead)
            }

            DecryptionFailed { aead } => {
                write!(f, "Decryption failed using {:?}", aead)
            }

            DerivationFailed { kdf } => {
                write!(f, "Key derivation failed using {:?}", kdf)
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

impl From<ed25519_dalek::ed25519::Error> for PrimitiveError {
    fn from(err: ed25519_dalek::ed25519::Error) -> Self {
        PrimitiveError::VerificationFailed {
            scheme: algorithms::SignatureScheme::Ed25519,
        }
    }
}
