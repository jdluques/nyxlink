use super::{backend::StorageBackend, entry::StorageEntry};
use crate::errors::CoreError;

pub(crate) struct StorageOrchestrator {
    memory: Box<dyn StorageBackend>,
    disk_plain: Box<dyn StorageBackend>,
    disk_encrypted: Box<dyn StorageBackend>,
}

impl StorageOrchestrator {
    fn backend_for<E: StorageEntry>(&self) -> &Box<dyn StorageBackend> {
        use super::backend::StorageBackendPolicy::*;

        match E::BACKEND {
            MemoryOnly => &self.memory,
            PersistentPlaintext => &self.disk_plain,
            PersistentEncrypted => &self.disk_encrypted,
        }
    }

    pub(crate) fn get<E: StorageEntry>(&self, id: &[u8]) -> Result<Option<E>, CoreError> {
        let backend = self.backend_for::<E>();

        match backend.get(E::KIND, id) {
            Ok(entry_bytes) => {
                let entry = E::decode(entry_bytes)?;
                Ok(Some(entry))
            }
            Err(err) => {
                log::info!("{}", err.to_string());
                Ok(None)
            }
        }
    }

    pub(crate) fn put<E: StorageEntry>(&self, entry: E) -> Result<Vec<u8>, CoreError> {
        let backend = self.backend_for::<E>();

        let kind = E::KIND;
        let id = &E::derive_id(&entry);
        let entry_bytes = E::encode(&entry);

        backend.put(kind, id, entry_bytes)?;

        Ok(id.to_vec())
    }

    pub(crate) fn delete<E: StorageEntry>(&self, id: &[u8]) -> Result<(), CoreError> {
        let backend = self.backend_for::<E>();

        backend.delete(E::KIND, id)?;

        Ok(())
    }
}
