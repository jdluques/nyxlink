use crate::{
    crypto::{
        rng::SecureRng,
        signature::{self, Signature},
    },
    errors::CoreError,
    identity::keys::PublicIdentity,
};

use super::keys::IdentityKeys;

pub(crate) struct Identity {
    keys: IdentityKeys,
}

impl Identity {
    pub fn new(rng: &mut SecureRng) -> Result<Self, CoreError> {
        Ok(Self {
            keys: IdentityKeys::new(rng)?,
        })
    }

    pub fn public(&self) -> Result<PublicIdentity, CoreError> {
        Ok(self.keys.public()?)
    }

    pub fn sign(&self, message: &[u8]) -> Result<Signature, CoreError> {
        signature::sign(self.keys.signing(), message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), CoreError> {
        Ok(signature::verify(
            self.keys.public()?.signing_key(),
            message,
            signature,
        )?)
    }
}
