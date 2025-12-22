use core::{error::Error, fmt};

use super::internal::{
    capability::CapabilityError, identity::IdentityError, input::InputError,
    primitive::PrimitiveError, protocol::ProtocolError, resource::ResourceError,
    storage::StorageError,
};

#[derive(Debug)]
#[allow(private_interfaces)]
pub enum CoreError {
    Primitive(PrimitiveError),
    Protocol(ProtocolError),
    Identity(IdentityError),
    Capability(CapabilityError),
    Storage(StorageError),
    InvalidInput(InputError),
    Resource(ResourceError),
    Internal,
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CoreError::*;

        match self {
            Primitive(_) => write!(f, "Primitive operation failed"),
            Protocol(_) => write!(f, "Protocol error"),
            Identity(_) => write!(f, "Identity error"),
            Capability(_) => write!(f, "Capability error"),
            Storage(_) => write!(f, "Storage error"),
            InvalidInput(_) => write!(f, "Invalid input"),
            Resource(_) => write!(f, "Resource error"),
            Internal => write!(f, "Internal error"),
        }
    }
}

impl Error for CoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use CoreError::*;

        match self {
            Primitive(e) => Some(e),
            Protocol(e) => Some(e),
            Identity(e) => Some(e),
            Capability(e) => Some(e),
            Storage(e) => Some(e),
            InvalidInput(e) => Some(e),
            Resource(e) => Some(e),
            Internal => None,
        }
    }
}

impl From<PrimitiveError> for CoreError {
    fn from(err: PrimitiveError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::Primitive(err)
    }
}

impl From<ProtocolError> for CoreError {
    fn from(err: ProtocolError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::Protocol(err)
    }
}

impl From<IdentityError> for CoreError {
    fn from(err: IdentityError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::Identity(err)
    }
}

impl From<CapabilityError> for CoreError {
    fn from(err: CapabilityError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::Capability(err)
    }
}

impl From<StorageError> for CoreError {
    fn from(err: StorageError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::Storage(err)
    }
}

impl From<InputError> for CoreError {
    fn from(err: InputError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::InvalidInput(err)
    }
}

impl From<ResourceError> for CoreError {
    fn from(err: ResourceError) -> Self {
        log::warn!("{}", err.to_string());
        CoreError::Resource(err)
    }
}
