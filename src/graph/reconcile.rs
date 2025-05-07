use crate::feature::BondKind;

pub fn reconcile(left: BondKind, right: &BondKind) -> Option<(BondKind, BondKind)> {
    use BondKind::{Down, Elided, Up};
    match (&left, &right) {
        (Up, Up) | (Down, Down) => None,
        (Up, Down) | (Down, Up) => Some((left, *right)),
        (Elided, Elided) => Some((Elided, Elided)),
        (Elided, Up) | (Down, Elided) => Some((Down, Up)),
        (Elided, Down) | (Up, Elided) => Some((Up, Down)),
        (other, Elided) | (Elided, &other) => Some((*other, *other)),
        (a, b) if a == *b => Some((*a, **b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_double() {
        assert_eq!(reconcile(BondKind::Single, &BondKind::Double), None)
    }

    #[test]
    fn up_up() {
        assert_eq!(reconcile(BondKind::Up, &BondKind::Up), None)
    }

    #[test]
    fn down_down() {
        assert_eq!(reconcile(BondKind::Down, &BondKind::Down), None)
    }

    #[test]
    fn elided_elided() {
        assert_eq!(
            reconcile(BondKind::Elided, &BondKind::Elided),
            Some((BondKind::Elided, BondKind::Elided))
        )
    }

    #[test]
    fn elided_single() {
        assert_eq!(
            reconcile(BondKind::Elided, &BondKind::Single),
            Some((BondKind::Single, BondKind::Single))
        )
    }

    #[test]
    fn elided_up() {
        assert_eq!(
            reconcile(BondKind::Elided, &BondKind::Up),
            Some((BondKind::Down, BondKind::Up))
        )
    }

    #[test]
    fn elided_down() {
        assert_eq!(
            reconcile(BondKind::Elided, &BondKind::Down),
            Some((BondKind::Up, BondKind::Down))
        )
    }

    #[test]
    fn up_elided() {
        assert_eq!(
            reconcile(BondKind::Up, &BondKind::Elided),
            Some((BondKind::Up, BondKind::Down))
        )
    }

    #[test]
    fn down_elided() {
        assert_eq!(
            reconcile(BondKind::Down, &BondKind::Elided),
            Some((BondKind::Down, BondKind::Up))
        )
    }

    #[test]
    fn up_down() {
        assert_eq!(
            reconcile(BondKind::Up, &BondKind::Down),
            Some((BondKind::Up, BondKind::Down))
        )
    }

    #[test]
    fn down_up() {
        assert_eq!(
            reconcile(BondKind::Down, &BondKind::Up),
            Some((BondKind::Down, BondKind::Up))
        )
    }

    #[test]
    fn single_elided() {
        assert_eq!(
            reconcile(BondKind::Single, &BondKind::Elided),
            Some((BondKind::Single, BondKind::Single))
        )
    }

    #[test]
    fn other_bonds() {
        assert_eq!(
            reconcile(BondKind::Triple, &BondKind::Triple),
            Some((BondKind::Triple, BondKind::Triple))
        )
    }
}
