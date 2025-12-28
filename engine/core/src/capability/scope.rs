#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityScope {
    EstablishSession,
    SendMessage,
    ReceiveMessage,
    RotateKeys,
    AccessStorage,
}

impl CapabilityScope {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::EstablishSession => "establish_session",
            Self::SendMessage => "send_message",
            Self::ReceiveMessage => "receive_message",
            Self::RotateKeys => "rotate_keys",
            Self::AccessStorage => "access_storage",
        }
    }
}
