#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WireMessageType {
    Handshake,
    Transport,
    Control,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WireVersion {
    V1,
}
