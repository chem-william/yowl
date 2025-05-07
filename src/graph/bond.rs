use crate::feature::BondKind;

/// A bond from a graph-like Atom to an Atom ID.
#[derive(Debug, PartialEq)]
pub struct Bond {
    pub kind: BondKind,
    pub tid: usize,
}

impl Bond {
    /// Constructs a Bond.
    pub fn new(kind: BondKind, tid: usize) -> Self {
        Self { kind, tid }
    }

    /// Returns the order of this Bond. Elided, Single, Up, Down,
    /// and Aromatic kinds return 1. The rest return the bond multiplicity.
    pub fn order(&self) -> u8 {
        match &self.kind {
            BondKind::Elided
            | BondKind::Single
            | BondKind::Up
            | BondKind::Down
            | BondKind::Aromatic => 1,
            BondKind::Double => 2,
            BondKind::Triple => 3,
            BondKind::Quadruple => 4,
        }
    }

    /// Returns true if this bond is encoded as Aromatic.
    pub fn is_aromatic(&self) -> bool {
        self.kind == BondKind::Aromatic
    }

    /// Returns true if this bond is Up or Down
    pub fn is_directional(&self) -> bool {
        self.kind == BondKind::Up || self.kind == BondKind::Down
    }
}
#[cfg(test)]
mod tests {
    use crate::{feature::BondKind, graph::Bond};

    #[test]
    fn test_order_elided_single_up_down_aromatic() {
        let kinds = [
            BondKind::Elided,
            BondKind::Single,
            BondKind::Up,
            BondKind::Down,
            BondKind::Aromatic,
        ];
        for kind in kinds.iter() {
            let bond = Bond::new(kind.clone(), 0);
            assert_eq!(bond.order(), 1, "{:?} should have order 1", kind);
        }
    }

    #[test]
    fn test_order_multiple() {
        let cases = [
            (BondKind::Single, 1),
            (BondKind::Double, 2),
            (BondKind::Triple, 3),
            (BondKind::Quadruple, 4),
        ];
        for (kind, expected) in cases.iter() {
            let bond = Bond::new(kind.clone(), 1);
            assert_eq!(
                bond.order(),
                *expected,
                "{:?} should have order {}",
                kind,
                expected
            );
        }
    }

    #[test]
    fn test_is_aromatic() {
        let aro = Bond::new(BondKind::Aromatic, 2);
        let non_aro = Bond::new(BondKind::Double, 2);
        assert!(aro.is_aromatic(), "Aromatic bond should be aromatic");
        assert!(!non_aro.is_aromatic(), "Double bond should not be aromatic");
    }

    #[test]
    fn test_is_directional() {
        let up = Bond::new(BondKind::Up, 3);
        let down = Bond::new(BondKind::Down, 4);
        let other = Bond::new(BondKind::Single, 5);
        assert!(up.is_directional(), "Up bond should be directional");
        assert!(down.is_directional(), "Down bond should be directional");
        assert!(
            !other.is_directional(),
            "Single bond should not be directional"
        );
    }

    #[test]
    fn test_partial_eq() {
        let b1 = Bond::new(BondKind::Single, 7);
        let b2 = Bond::new(BondKind::Single, 7);
        let b3 = Bond::new(BondKind::Single, 8);
        assert_eq!(b1, b2);
        assert_ne!(b1, b3);
    }
}
