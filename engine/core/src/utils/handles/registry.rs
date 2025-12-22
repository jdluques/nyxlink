use core::num::NonZeroUsize;

use super::types::Handle;
use crate::errors::internal::StorageError;

pub(crate) struct HandleRegistry<T> {
    next_id: NonZeroUsize,
    entries: Vec<T>,
}

impl<T> HandleRegistry<T> {
    pub(crate) fn new() -> Self {
        Self {
            next_id: NonZeroUsize::new(1).unwrap(),
            entries: Vec::new(),
        }
    }

    pub(crate) fn insert(&mut self, value: T) -> Result<Handle<T>, StorageError> {
        let id = self.next_id;
        self.next_id
            .checked_add(1)
            .ok_or(StorageError::OutOfSpace)?;

        let handle = Handle::new(id);
        self.entries.push(value);

        Ok(handle)
    }

    pub(crate) fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.entries.get(handle.as_usize())
    }

    pub(crate) fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.entries.get_mut(handle.as_usize())
    }

    pub(crate) fn remove(&mut self, handle: Handle<T>) -> Option<&T> {
        self.entries.get(handle.as_usize()).take()
    }
}
