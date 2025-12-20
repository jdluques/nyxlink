use core::{error::Error, fmt};

#[derive(Debug)]
pub(crate) enum CapabilityError {
    Invalid { context: &'static str },
    Expired,
    Unauthorized,
    Revoked,
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CapabilityError::*;

        match self {
            Invalid { context } => {
                write!(f, "Invalid capability: {}", context)
            }
            Expired => {
                write!(f, "Capability expired")
            }
            Unauthorized => {
                write!(f, "Capability unauthorized")
            }
            Revoked => {
                write!(f, "Capability revoked")
            }
        }
    }
}

impl Error for CapabilityError {}
