use super::Bond;
use crate::feature::AtomKind;

/// Atom used in graph-like (adjacency) SMILES representation.
#[derive(Debug, PartialEq)]
pub struct Atom {
    pub kind: AtomKind,
    pub bonds: Vec<Bond>,
}

impl Atom {
    /// Constructs an Atom without bonds.
    pub fn new(kind: AtomKind) -> Self {
        Self {
            kind,
            bonds: vec![],
        }
    }

    /// Returns true if the atom was encoded as aromatic.
    pub fn is_aromatic(&self) -> bool {
        self.kind.is_aromatic()
    }

    /// Computes and returns the subvalence associated with this Atom.
    /// Subvalence represents the maximum number of [implicit hydrogens](https://depth-first.com/articles/2020/06/08/hydrogen-suppression-in-smiles/)
    /// that can be added to this Atom without exceeding a valence target.
    /// This value is independent of an atom's aromaticity marking.
    pub fn subvalence(&self) -> u8 {
        let hcount = match &self.kind {
            AtomKind::Bracket {
                hcount: Some(h), ..
            } => h.into(),
            _ => 0,
        };

        let valence = self
            .bonds
            .iter()
            .fold(hcount, |sum, bond| sum + bond.order());
        self.kind
            .targets()
            .iter()
            .find(|&&target| target >= valence)
            .map_or(0, |&target| target - valence)
    }

    /// Returns the number of implicit or virtual hydrogens at this Atom,
    /// accounting for aromaticity.
    pub fn suppressed_hydrogens(&self) -> u8 {
        let subvalence = self.subvalence();
        match &self.kind {
            AtomKind::Star => 0,
            AtomKind::Aromatic(_) => {
                if subvalence > 1 {
                    subvalence - 1
                } else {
                    0
                }
            }
            AtomKind::Aliphatic(_) => subvalence,
            AtomKind::Bracket { hcount, .. } => match hcount {
                Some(hcount) => hcount.into(),
                None => 0,
            },
        }
    }
}

#[cfg(test)]
mod subvalence {
    use super::*;
    use crate::feature::{
        Aliphatic, Aromatic, BondKind, BracketAromatic, BracketSymbol, Charge, Element,
        VirtualHydrogen,
    };

    #[test]
    fn star() {
        let atom = Atom {
            kind: AtomKind::Star,
            bonds: vec![],
        };

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn star_single() {
        let atom = Atom {
            kind: AtomKind::Star,
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn carbon_single() {
        let atom = Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn aromatic_carbon_single() {
        let atom = Atom {
            kind: AtomKind::Aromatic(Aromatic::C),
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_star_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: None,
                charge: None,
                map: None,
            },
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn bracket_carbon_h1() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                configuration: None,
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                map: None,
            },
            bonds: vec![],
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_carbon_h0_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                configuration: None,
                hcount: None,
                charge: None,
                map: None,
            },
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_aromatic_h0_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::C),
                configuration: None,
                hcount: None,
                charge: None,
                map: None,
            },
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_aromatic_carbon_h1_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::C),
                configuration: None,
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                map: None,
            },
            bonds: vec![Bond::new(BondKind::Single, 1)],
        };

        assert_eq!(atom.subvalence(), 2)
    }

    #[test]
    fn sulfur_charged_divalent() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::S),
                configuration: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None,
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1),
                Bond::new(BondKind::Single, 2),
            ],
        };

        assert_eq!(atom.subvalence(), 1)
    }
}

#[cfg(test)]
mod suppressed_hydrogens {
    use super::*;
    use crate::feature::{Aliphatic, Aromatic, BondKind, BracketSymbol, Element, VirtualHydrogen};
    use pretty_assertions::assert_eq;

    #[test]
    fn star() {
        let atom = Atom::new(AtomKind::Star);

        assert_eq!(atom.suppressed_hydrogens(), 0)
    }

    #[test]
    fn aromatic_subvalence_1() {
        let atom = Atom {
            kind: AtomKind::Aromatic(Aromatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 1),
                Bond::new(BondKind::Elided, 2),
                Bond::new(BondKind::Elided, 3),
            ],
        };

        assert_eq!(atom.suppressed_hydrogens(), 0)
    }

    #[test]
    fn aromatic_subvalence_2() {
        let atom = Atom {
            kind: AtomKind::Aromatic(Aromatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 1),
                Bond::new(BondKind::Elided, 2),
            ],
        };

        assert_eq!(atom.suppressed_hydrogens(), 1)
    }

    #[test]
    fn aliphatic_subvalence_0() {
        let atom = Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 1),
                Bond::new(BondKind::Elided, 2),
                Bond::new(BondKind::Elided, 3),
                Bond::new(BondKind::Elided, 4),
            ],
        };

        assert_eq!(atom.suppressed_hydrogens(), 0)
    }

    #[test]
    fn aliphatic_subvalence_1() {
        let atom = Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 1),
                Bond::new(BondKind::Elided, 2),
                Bond::new(BondKind::Elided, 3),
            ],
        };

        assert_eq!(atom.suppressed_hydrogens(), 1)
    }

    #[test]
    fn aliphatic_subvalence_2() {
        let atom = Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 1),
                Bond::new(BondKind::Elided, 2),
            ],
        };

        assert_eq!(atom.suppressed_hydrogens(), 2)
    }

    #[test]
    fn bracket_h_none() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: None,
                charge: None,
                configuration: None,
                map: None,
            },
            bonds: vec![],
        };

        assert_eq!(atom.suppressed_hydrogens(), 0)
    }

    #[test]
    fn bracket_h0() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: Some(VirtualHydrogen::H0),
                charge: None,
                configuration: None,
                map: None,
            },
            bonds: vec![],
        };

        assert_eq!(atom.suppressed_hydrogens(), 0)
    }

    #[test]
    fn bracket_h1() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                configuration: None,
                map: None,
            },
            bonds: vec![],
        };

        assert_eq!(atom.suppressed_hydrogens(), 1)
    }
}
