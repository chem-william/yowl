use std::convert::TryFrom;
use std::fmt;

use super::{Aliphatic, BracketAromatic};

/// Atomic symbols that can be aromatic.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Aromatic {
    B,
    C,
    N,
    O,
    P,
    S,
}

impl Aromatic {
    /// The valence targets available to this aromatic.
    pub const fn targets(&self) -> &[u8] {
        match self {
            Self::B => &[3],
            Self::C => &[4],
            Self::N => &[3, 5],
            Self::O => &[2],
            Self::P => &[3, 5],
            Self::S => &[2, 4, 6],
        }
    }
}

impl TryFrom<&BracketAromatic> for Aromatic {
    type Error = ();

    fn try_from(value: &BracketAromatic) -> Result<Self, Self::Error> {
        match value {
            BracketAromatic::B => Ok(Self::B),
            BracketAromatic::C => Ok(Self::C),
            BracketAromatic::N => Ok(Self::N),
            BracketAromatic::O => Ok(Self::O),
            BracketAromatic::P => Ok(Self::P),
            BracketAromatic::S => Ok(Self::S),
            _ => Err(()),
        }
    }
}

impl From<&Aromatic> for Aliphatic {
    fn from(val: &Aromatic) -> Self {
        match val {
            Aromatic::B => Self::B,
            Aromatic::C => Self::C,
            Aromatic::N => Self::N,
            Aromatic::O => Self::O,
            Aromatic::P => Self::P,
            Aromatic::S => Self::S,
        }
    }
}

impl fmt::Display for Aromatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::B => "b",
                Self::C => "c",
                Self::N => "n",
                Self::O => "o",
                Self::P => "p",
                Self::S => "s",
            }
        )
    }
}
