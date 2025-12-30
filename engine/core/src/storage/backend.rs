use crate::{errors::internal::StorageError, storage::entry::StorageEntryKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StorageBackendPolicy {
    MemoryOnly,
    PersistentPlaintext,
    PersistentEncrypted,
}

pub(super) trait StorageBackend: Send + Sync {
    fn get(&self, kind: StorageEntryKind, id: &[u8]) -> Result<&[u8], StorageError>;

    fn put(
        &self,
        kind: StorageEntryKind,
        id: &[u8],
        entry_bytes: &[u8],
    ) -> Result<(), StorageError>;

    fn delete(&self, kind: StorageEntryKind, id: &[u8]) -> Result<(), StorageError>;
}
