use chacha20poly1305::{
    self, ChaCha20Poly1305, Key, KeyInit, Nonce,
    aead::{Aead, Payload},
};

use crate::{
    crypto::algorithms,
    errors::{
        CoreError,
        internal::{InputError, PrimitiveError},
    },
};

pub fn encrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CoreError> {
    if aad.is_empty() {
        Err(InputError::InvalidValue {
            context: "Additional Authenticated Data must not be empty",
        })?
    }

    if plaintext.is_empty() {
        Err(InputError::InvalidValue {
            context: "Plaintext must not be empty",
        })?
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    Ok(cipher
        .encrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|_| PrimitiveError::EncryptionFailed {
            aead: algorithms::AEAD::ChaCha20Poly1305,
        })?)
}

pub fn decrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CoreError> {
    if aad.is_empty() {
        Err(InputError::InvalidValue {
            context: "Additional Authenticated Data must not be empty",
        })?
    }

    if ciphertext.is_empty() {
        Err(InputError::InvalidValue {
            context: "Ciphertext must not be empty",
        })?
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    Ok(cipher
        .decrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| PrimitiveError::DecryptionFailed {
            aead: algorithms::AEAD::ChaCha20Poly1305,
        })?)
}
