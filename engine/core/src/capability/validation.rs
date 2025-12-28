use crate::{
    crypto::{keys::signing::PublicSigningKey, signature::verify},
    errors::internal::CapabilityError,
};

use super::{capability::Capability, token::CapabilityToken};

#[allow(private_interfaces)]
pub fn validate_token(
    token: &CapabilityToken,
    issuer_pk: &PublicSigningKey,
    now: u64,
) -> Result<Capability, CapabilityError> {
    if let Err(err) = verify(issuer_pk, &token.signing_bytes(), &token.signature) {
        log::warn!("{}", err.to_string());
        return Err(CapabilityError::Invalid {
            context: "Signature verification failed",
        });
    };

    if token.constraints.is_expired(now) {
        return Err(CapabilityError::Expired);
    }

    Ok(Capability {
        scopes: token.scopes.clone(),
        constraints: token.constraints.clone(),
        issued_at: now,
        uses: 0,
    })
}
