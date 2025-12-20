use core::{error::Error, fmt};

#[derive(Debug)]
pub(crate) enum ResourceError {
    OutOfMemory,
    Exhausted { resource: &'static str },
    SystemFailure { context: &'static str },
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResourceError::*;

        match self {
            OutOfMemory => {
                write!(f, "Out of memory")
            }
            Exhausted { resource } => {
                write!(f, "Resource exhausted: {}", resource)
            }
            SystemFailure { context } => {
                write!(f, "System failure: {}", context)
            }
        }
    }
}

impl Error for ResourceError {}
