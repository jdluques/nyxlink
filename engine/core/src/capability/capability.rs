use crate::errors::internal::CapabilityError;

use super::{constraint::CapabilityConstraints, scope::CapabilityScope};

#[derive(Debug)]
pub struct Capability {
    pub scopes: Vec<CapabilityScope>,
    pub constraints: CapabilityConstraints,

    pub issued_at: u64,

    pub uses: u64,
}

impl Capability {
    pub fn allows(&self, scope: CapabilityScope) -> bool {
        self.scopes.contains(&scope)
    }

    pub fn consume(&mut self) -> Result<(), CapabilityError> {
        if let Some(max) = self.constraints.max_uses {
            if self.uses >= max {
                return Err(CapabilityError::Expired);
            }
        }
        self.uses += 1;
        Ok(())
    }
}
