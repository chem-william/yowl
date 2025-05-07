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
            BracketAromatic::As => Element::As,
            BracketAromatic::B => Element::B,
            BracketAromatic::C => Element::C,
            BracketAromatic::N => Element::N,
            BracketAromatic::O => Element::O,
            BracketAromatic::P => Element::P,
            BracketAromatic::S => Element::S,
            BracketAromatic::Se => Element::Se,
            BracketAromatic::Si => Element::Si,
            BracketAromatic::Te => Element::Te,
        }
    }
}

impl fmt::Display for BracketAromatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BracketAromatic::B => "b",
                BracketAromatic::C => "c",
                BracketAromatic::N => "n",
                BracketAromatic::O => "o",
                BracketAromatic::S => "s",
                BracketAromatic::P => "p",
                BracketAromatic::Se => "se",
                BracketAromatic::As => "as",
                BracketAromatic::Si => "si",
                BracketAromatic::Te => "te",
            }
        )
    }
}
