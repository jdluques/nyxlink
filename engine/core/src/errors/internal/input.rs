use core::{error::Error, fmt};

#[derive(Debug)]
pub(crate) enum InputError {
    InvalidLength { expected: usize, actual: usize },
    InvalidFormat { context: &'static str },
    InvalidValue { context: &'static str },
    MissingValue { context: &'static str },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InputError::*;

        match self {
            InvalidLength { expected, actual } => {
                write!(f, "Invalid length: expected {}, got {}", expected, actual)
            }
            InvalidFormat { context } => {
                write!(f, "Invalid format: {}", context)
            }
            InvalidValue { context } => {
                write!(f, "Invalid value: {}", context)
            }
            MissingValue { context } => {
                write!(f, "Missing value: {}", context)
            }
        }
    }
}

impl Error for InputError {}
