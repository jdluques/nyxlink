pub mod kind;

pub(crate) use self::kind::StorageEntryKind;

use super::backend::StorageBackendPolicy;
use crate::errors::CoreError;

pub(super) trait StorageEntry: Sized {
    const KIND: StorageEntryKind;
    const BACKEND: StorageBackendPolicy;

    fn derive_id(&self) -> Vec<u8>;

    fn encode(&self) -> &[u8];
    fn decode(bytes: &[u8]) -> Result<Self, CoreError>;
}
