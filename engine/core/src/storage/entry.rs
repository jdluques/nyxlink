use core::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct StorageEntryId(u64);

#[derive(Debug, Copy, Clone)]
pub(crate) enum StorageEntryKind {
    Identity,
    Session,
    Capability,
    PreKey,
    MessageState,
}

impl fmt::Display for StorageEntryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StorageEntryKind::*;

        match self {
            Identity => write!(f, "Identity"),
            Session => write!(f, "Session"),
            Capability => write!(f, "Capability"),
            PreKey => write!(f, "PreKey"),
            MessageState => write!(f, "Message state"),
        }
    }
}
