use std::fmt;

use mendeleev::Element;

use super::atom_kind;

/// Atomic symbols that can be aromatic.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Aromatic {
    B,
    C,
    N,
    O,
    P,
    S,
    Se,
    As,
    Si,
    Te,
}

impl Aromatic {
    /// The valence targets available to this aromatic.
    pub fn targets(&self) -> &[u8] {
        atom_kind::elemental_targets(self.into(), None)
    }
}

impl From<&Aromatic> for Element {
    fn from(val: &Aromatic) -> Self {
        match val {
            Aromatic::B => Self::B,
            Aromatic::C => Self::C,
            Aromatic::N => Self::N,
            Aromatic::O => Self::O,
            Aromatic::P => Self::P,
            Aromatic::S => Self::S,
            Aromatic::Se => Self::Se,
            Aromatic::As => Self::As,
            Aromatic::Si => Self::Si,
            Aromatic::Te => Self::Te,
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
                Self::Se => "se",
                Self::As => "as",
                Self::Si => "si",
                Self::Te => "te",
            }
        )
    }
}
