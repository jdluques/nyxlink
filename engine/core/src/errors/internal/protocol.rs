use core::{error::Error, fmt};

use crate::protocol::{
    noise::{
        pattern::NoisePattern,
        state::{HandshakeFailure, HandshakeState},
    },
    ratchet::state::RatchetStep,
    wire::types::{WireMessageType, WireVersion},
};

#[derive(Debug)]
pub(crate) enum ProtocolError {
    HandshakeFailed {
        pattern: NoisePattern,
        reason: HandshakeFailure,
    },
    RatchetFailed {
        step: RatchetStep,
        reason: &'static str,
    },
    UnexpectedMessage {
        state: HandshakeState,
        message_type: WireMessageType,
        version: WireVersion,
    },
    InvalidState {
        context: &'static str,
    },
    SessionClosed,
    AuthenticationFailed,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ProtocolError::*;

        match self {
            HandshakeFailed { pattern, reason } => {
                write!(f, "{:?} handshake failed: {}", pattern, reason)
            }
            RatchetFailed { step, reason } => {
                write!(f, "Ratched failed in step {:?}: {}", step, reason)
            }
            #[allow(unused_variables)]
            UnexpectedMessage {
                state,
                message_type,
                version,
            } => {
                write!(f, "Unexpected message in state {}", state)
            }
            InvalidState { context } => {
                write!(f, "Invalid protocol state: {}", context)
            }
            SessionClosed => {
                write!(f, "Session closed")
            }
            AuthenticationFailed => {
                write!(f, "Authentication failed")
            }
        }
    }
}

impl Error for ProtocolError {}
