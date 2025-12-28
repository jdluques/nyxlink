use core::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityConstraints {
    pub expires_at: Option<u64>,
    pub max_uses: Option<u64>,
    pub bound_identity: Option<[u8; 32]>,
    pub bound_session: Option<[u8; 32]>,
}

impl CapabilityConstraints {
    pub fn is_expired(&self, now: u64) -> bool {
        self.expires_at.map_or(false, |t| now >= t)
    }

    pub fn allows_identity(&self, identity: &[u8; 32]) -> bool {
        self.bound_identity.map_or(true, |id| &id == identity)
    }

    pub fn allows_session(&self, session: &[u8; 32]) -> bool {
        self.bound_session.map_or(true, |id| &id == session)
    }

    pub fn attenuate(&self, other: &Self) -> Option<Self> {
        Some(Self {
            expires_at: match (self.expires_at, other.expires_at) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            max_uses: match (self.max_uses, other.max_uses) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            bound_identity: other.bound_identity.or(self.bound_identity),
            bound_session: other.bound_session.or(self.bound_session),
        })
    }
}
