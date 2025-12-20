use core::{error::Error, fmt};

use crate::storage::entry::{StorageEntryId, StorageEntryKind};

#[derive(Debug)]
pub(crate) enum StorageError {
    NotFound {
        kind: StorageEntryKind,
        id: StorageEntryId,
    },
    Corrupted {
        kind: StorageEntryKind,
        id: StorageEntryId,
        context: &'static str,
    },
    PermissionDenied {
        kind: StorageEntryKind,
        id: StorageEntryId,
    },
    Unavailable,
    OutOfSpace,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StorageError::*;

        match self {
            #[allow(unused_variables)]
            NotFound { kind, id } => {
                write!(f, "Storage entry not found ({})", kind)
            }
            #[allow(unused_variables)]
            Corrupted { kind, id, context } => {
                write!(f, "Storage corrupted ({}): {}", kind, context)
            }
            #[allow(unused_variables)]
            PermissionDenied { kind, id } => {
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
