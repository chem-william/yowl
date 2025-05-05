use thiserror::Error;

/// An error resulting from depth-first traversal of a graph
/// representation.
#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("A bond is missing its counterpart: ({0}, {1})")]
    HalfBond(usize, usize),
    #[error("A bond is duplicated: ({0}, {1})")]
    DuplicateBond(usize, usize),
    #[error("The target of a bond is unknown: ({0}, {1})")]
    UnknownTarget(usize, usize),
    #[error("A bond is incompatible: ({0}, {1})")]
    IncompatibleBond(usize, usize),
    #[error("A loop was detected at node: {0}")]
    Loop(usize),
}
