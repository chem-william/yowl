use thiserror::Error;

/// An error that occurs when reading a SMILES string.
#[derive(Debug, PartialEq, Error)]
pub enum ReadError {
    #[error("Unexpected end of input")]
    EndOfLine,
    #[error("Unexpected character: {0}")]
    Character(usize),
}
