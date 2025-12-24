use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum StorageEntryKind {
    Capability,
    Identity,
    Session,
    PreKey,
    Config,
    InternalMetadata,
}

impl fmt::Display for StorageEntryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StorageEntryKind::*;

        match self {
            Capability => write!(f, "Capability"),
            Identity => write!(f, "Identity"),
            Session => write!(f, "Session"),
            PreKey => write!(f, "PreKey"),
            Config => write!(f, "Config"),
            InternalMetadata => write!(f, "Internal metadata"),
        }
    }
}
