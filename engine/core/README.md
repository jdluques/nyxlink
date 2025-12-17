# Nyxlink Engine Core

Nyxlink Engine Core is a Rust crate that implements the cryptographic core of a secure P2P messaging application. It provides:

- Noise protocol handshakes (XXpsk2 and IKpsk2)
- Session state management with forward secrecy ratchets
- Identity management (ED25519 / X25519 keys)
- Encrypted storage for identities and offline messages
- Capability/invite system with validation
- Secure memory handling (zeroization, mlock)
- Constant-time operations for sensitive comparisons

> **Note:** This crate **does not handle FFI** directly. The Bridge create exposes a safe C ABI for use by the runtime. All cryptograhic operations and session management are performed here in Rust-native types.

## Security Considerations

- All sensitive buffers are zeroized after use
- Forward secrecy ratchets are applied per session
- Constant-time operations protect against timing attacks.
- Designed for integration with a Brifge FFI; panic abort is safe because Core does not unwind across FFI boundaries
