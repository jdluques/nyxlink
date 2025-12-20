pub mod core;
pub(crate) mod internal {
    pub(crate) mod capability;
    pub(crate) mod identity;
    pub(crate) mod input;
    pub(crate) mod primitive;
    pub(crate) mod protocol;
    pub(crate) mod resource;
    pub(crate) mod storage;

    pub(crate) use self::{
        capability::CapabilityError, identity::IdentityError, input::InputError,
        primitive::PrimitiveError, protocol::ProtocolError, resource::ResourceError,
        storage::StorageError,
    };
}

pub use core::CoreError;
