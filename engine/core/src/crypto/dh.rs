use x25519_dalek::SharedSecret;

use crate::{
    crypto::keys::dh::{PrivateDHKey, PublicDHKey},
    errors::CoreError,
};

pub fn diffie_hellman(
    private: &PrivateDHKey,
    public: &PublicDHKey,
) -> Result<SharedSecret, CoreError> {
    Ok(private.inner()?.diffie_hellman(public.inner()))
}
