use crate::{
    crypto::{
        keys::{
            dh::{PrivateDHKey, PublicDHKey},
            signing::{PrivateSigningKey, PublicSigningKey},
        },
        rng::SecureRng,
    },
    errors::CoreError,
};

pub(crate) struct IdentityKeys {
    signing_key: PrivateSigningKey,
    dh_key: PrivateDHKey,
}

#[derive(Clone, Debug)]
pub(crate) struct PublicIdentity {
    pub signing_pk: PublicSigningKey,
    pub dh_pk: PublicDHKey,
}

impl IdentityKeys {
    pub fn new(rng: &mut SecureRng) -> Result<Self, CoreError> {
        Ok(Self {
            signing_key: PrivateSigningKey::generate(rng)?,
            dh_key: PrivateDHKey::generate(rng)?,
        })
    }

    pub fn public(&self) -> Result<PublicIdentity, CoreError> {
        Ok(PublicIdentity {
            signing_pk: self.signing_key.public_key()?,
            dh_pk: self.dh_key.public_key()?,
        })
    }

    pub(crate) fn signing(&self) -> &PrivateSigningKey {
        &self.signing_key
    }
}

impl PublicIdentity {
    pub fn signing_key(&self) -> &PublicSigningKey {
        &self.signing_pk
    }

    pub fn dh_key(&self) -> &PublicDHKey {
        &self.dh_pk
    }
}
