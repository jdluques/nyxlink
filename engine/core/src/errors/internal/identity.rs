use core::{error::Error, fmt};

#[derive(Debug)]
pub(crate) enum IdentityError {
    NotFound,
    AlreadyExists,
    Corrupted { context: &'static str },
    Locked,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IdentityError::*;

        match self {
            NotFound => {
                write!(f, "Identity not found")
            }
            AlreadyExists => {
                write!(f, "Identity already exists")
            }
            Corrupted { context } => {
                write!(f, "Identity corrupted: {}", context)
            }
            Locked => {
                write!(f, "Identity is locked")
            }
        }
    }
}

impl Error for IdentityError {}
