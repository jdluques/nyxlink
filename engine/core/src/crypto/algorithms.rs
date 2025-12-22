#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum DHGroup {
    X25519,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum SignatureScheme {
    Ed25519,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AEAD {
    ChaCha20Poly1305,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Hash {
    SHA512,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum KDF {
    HKDF_SHA256,
}
