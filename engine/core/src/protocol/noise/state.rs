use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HandshakeState {
    Init,
    Message1,
    Message2,
    Message3,
    Transport,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HandshakeFailure {
    InvalidPsk,
    InvalidKey,
    ReplayDetected,
    MessageOutOfOrder,
}

impl fmt::Display for HandshakeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HandshakeState::*;

        match self {
            Init => write!(f, "Init"),
            Message1 => write!(f, "Message 1"),
            Message2 => write!(f, "Message 2"),
            Message3 => write!(f, "Message 3"),
            Transport => write!(f, "Transport"),
            Failed => write!(f, "Failed"),
        }
    }
}

impl fmt::Display for HandshakeFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HandshakeFailure::*;

        match self {
            InvalidPsk => write!(f, "Invalid psk"),
            InvalidKey => write!(f, "Invalid key"),
            ReplayDetected => write!(f, "Replay detected"),
            MessageOutOfOrder => write!(f, "Message out of order"),
        }
    }
}
