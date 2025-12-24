use core::{error::Error, fmt};

use crate::storage::entry::StorageEntryKind;

#[derive(Debug)]
pub(crate) enum StorageError {
    NotFound {
        kind: StorageEntryKind,
    },
    Corrupted {
        kind: StorageEntryKind,
        context: &'static str,
    },
    PermissionDenied {
        kind: StorageEntryKind,
    },
    Unavailable,
    OutOfSpace,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StorageError::*;

        match self {
            NotFound { kind } => {
                write!(f, "Storage entry not found ({})", kind)
            }
            Corrupted { kind, context } => {
                write!(f, "Storage corrupted ({}): {}", kind, context)
            }
            PermissionDenied { kind } => {
                write!(f, "Storage permission denied ({})", kind)
            }
            Unavailable => {
                write!(f, "Storage unavailable")
            }
            OutOfSpace => {
                write!(f, "Storage out of space")
            }
        }
    }
}

impl Error for StorageError {}
