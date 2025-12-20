#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum LockStrategy {
    None,
    BestEffort,
    Required,
}
