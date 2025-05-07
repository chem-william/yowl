use std::fmt;

use super::Element;

/// Eligible symbols for aromatic bracket atoms.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BracketAromatic {
    B,
    C,
    N,
    O,
    S,
    P,
    Se,
    As,
    Si,
    Te,
}

impl From<&BracketAromatic> for Element {
    fn from(val: &BracketAromatic) -> Self {
        match val {
            BracketAromatic::As => Self::As,
            BracketAromatic::B => Self::B,
            BracketAromatic::C => Self::C,
            BracketAromatic::N => Self::N,
            BracketAromatic::O => Self::O,
            BracketAromatic::P => Self::P,
            BracketAromatic::S => Self::S,
            BracketAromatic::Se => Self::Se,
            BracketAromatic::Si => Self::Si,
            BracketAromatic::Te => Self::Te,
        }
    }
}

impl fmt::Display for BracketAromatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::B => "b",
                Self::C => "c",
                Self::N => "n",
                Self::O => "o",
                Self::S => "s",
                Self::P => "p",
                Self::Se => "se",
                Self::As => "as",
                Self::Si => "si",
                Self::Te => "te",
            }
        )
    }
}
