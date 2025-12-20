#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ZeroizePolicy {
    OnDrop,
    Explicit,
}
