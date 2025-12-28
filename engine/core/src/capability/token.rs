use super::{constraint::CapabilityConstraints, scope::CapabilityScope};
use crate::crypto::signature::Signature;

#[derive(Debug, Clone)]
pub struct CapabilityToken {
    pub issuer: [u8; 32],
    pub subject: [u8; 32],

    pub scopes: Vec<CapabilityScope>,
    pub constraints: CapabilityConstraints,

    pub signature: Signature,
}

impl CapabilityToken {
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend_from_slice(&self.issuer);
        out.extend_from_slice(&self.subject);

        let mut sorted_scopes = self.scopes.clone();
        sorted_scopes.sort_by_key(|s| s.as_str());
        for scope in &sorted_scopes {
            let s_bytes = scope.as_str().as_bytes();
            out.extend_from_slice(&(s_bytes.len() as u32).to_be_bytes());
            out.extend_from_slice(s_bytes);
        }

        out.push(self.constraints.expires_at.is_some() as u8);
        if let Some(t) = self.constraints.expires_at {
            out.extend_from_slice(&t.to_be_bytes());
        }

        out.push(self.constraints.max_uses.is_some() as u8);
        if let Some(m) = self.constraints.max_uses {
            out.extend_from_slice(&m.to_be_bytes());
        }

        out.push(self.constraints.bound_identity.is_some() as u8);
        if let Some(id) = &self.constraints.bound_identity {
            out.extend_from_slice(id);
        }

        out.push(self.constraints.bound_session.is_some() as u8);
        if let Some(id) = &self.constraints.bound_session {
            out.extend_from_slice(id);
        }

        out
    }
}
