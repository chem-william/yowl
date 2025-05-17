use crate::Element;

/// Represents those atomic symbols capable of appearing within a bracket
/// atom in the string representation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BracketSymbol {
    Star,
    Element(Element),
    Aromatic(Element),
}
