use std::fmt;

use super::{BracketAromatic, Element};

/// Represents those atomic symbols capable of appearing within a bracket
/// atom in the string representation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BracketSymbol {
    Star,
    Element(Element),
    Aromatic(BracketAromatic),
}

impl fmt::Display for BracketSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Star => write!(f, "*"),
            Self::Aromatic(aromatic) => write!(f, "{aromatic}"),
            Self::Element(element) => write!(f, "{element}"),
        }
    }
}
