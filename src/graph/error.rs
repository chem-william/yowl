use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("join error")]
    Join(usize, usize),
    #[error("rnum error")]
    Rnum(usize),
}
