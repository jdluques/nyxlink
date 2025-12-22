use core::{fmt, hash, marker::PhantomData, num::NonZeroUsize};

#[repr(transparent)]
pub struct Handle<T> {
    id: NonZeroUsize,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub(crate) fn new(id: NonZeroUsize) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn as_usize(self) -> usize {
        self.id.get()
    }

    pub(crate) fn from_usize(raw: usize) -> Option<Self> {
        NonZeroUsize::new(raw).map(Self::new)
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Handle<T> {}

impl<T> hash::Hash for Handle<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Handle").field("id", &self.id).finish()
    }
}

impl<T> fmt::Display for Handle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "handle({})", self.id)
    }
}
