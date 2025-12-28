use super::{constraint::CapabilityConstraints, scope::CapabilityScope, token::CapabilityToken};
use crate::{
    crypto::{
        keys::signing::PrivateSigningKey,
        signature::{self, Signature},
    },
    errors::{CoreError, internal::CapabilityError},
};

#[derive(Clone, Debug)]
pub struct DelegationToken {
    pub parent_signature: Signature,
    pub scopes: Vec<CapabilityScope>,
    pub constraints: CapabilityConstraints,
    pub signature: Signature,
}

pub fn delegate(
    parent: &CapabilityToken,
    new_scopes: Vec<CapabilityScope>,
    new_constraints: CapabilityConstraints,
    issuer_key: &PrivateSigningKey,
) -> Result<DelegationToken, CoreError> {
    for scope in &new_scopes {
        if !parent.scopes.contains(scope) {
            return Err(CapabilityError::Invalid {
                context: "Scope not contained in parent",
            })?;
        }
    }

    let constraints =
        parent
            .constraints
            .attenuate(&new_constraints)
            .ok_or(CapabilityError::Invalid {
                context: "Failed to attenuate constraints",
            })?;

    let mut payload = Vec::new();

    payload.extend_from_slice(parent.signature.as_bytes());

    let mut sorted_scopes = new_scopes.clone();
    sorted_scopes.sort_by_key(|s| s.as_str());
    for scope in &sorted_scopes {
        let s_bytes = scope.as_str().as_bytes();
        payload.extend_from_slice(&(s_bytes.len() as u32).to_be_bytes());
        payload.extend_from_slice(s_bytes);
    }

    payload.push(constraints.expires_at.is_some() as u8);
    if let Some(t) = &constraints.expires_at {
        payload.extend_from_slice(&t.to_be_bytes());
    }

    payload.push(constraints.max_uses.is_some() as u8);
    if let Some(m) = &constraints.max_uses {
        payload.extend_from_slice(&m.to_be_bytes());
    }

    payload.push(constraints.bound_identity.is_some() as u8);
    if let Some(id) = &constraints.bound_identity {
        payload.extend_from_slice(id);
    }

    payload.push(constraints.bound_session.is_some() as u8);
    if let Some(id) = &constraints.bound_session {
        payload.extend_from_slice(id);
    }

    let signature = signature::sign(issuer_key, &payload)?;

    Ok(DelegationToken {
        parent_signature: parent.signature.clone(),
        scopes: new_scopes,
        constraints,
        signature,
    })
}
