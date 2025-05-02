use crate::feature::BondKind;

pub fn reconcile(left: BondKind, right: BondKind) -> Option<(BondKind, BondKind)> {
    if left == right {
        if left == BondKind::Up || left == BondKind::Down {
            None
        } else {
            Some((left, right))
        }
    } else if left == BondKind::Up && right == BondKind::Down {
        Some((left, right))
    } else if left == BondKind::Down && right == BondKind::Up {
        Some((left, right))
    } else if left == BondKind::Elided {
        match right {
            BondKind::Up => Some((BondKind::Down, right)),
            BondKind::Down => Some((BondKind::Up, right)),
            _ => Some((right.clone(), right)),
        }
    } else if right == BondKind::Elided {
        match left {
            BondKind::Up => Some((left, BondKind::Down)),
            BondKind::Down => Some((left, BondKind::Up)),
            _ => Some((left.clone(), left)),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_double() {
        assert_eq!(reconcile(BondKind::Single, BondKind::Double), None)
    }

    #[test]
    fn up_up() {
        assert_eq!(reconcile(BondKind::Up, BondKind::Up), None)
    }

    #[test]
    fn down_down() {
        assert_eq!(reconcile(BondKind::Down, BondKind::Down), None)
    }

    #[test]
    fn elided_elided() {
        assert_eq!(
            reconcile(BondKind::Elided, BondKind::Elided),
            Some((BondKind::Elided, BondKind::Elided))
        )
    }

    #[test]
    fn elided_single() {
        assert_eq!(
            reconcile(BondKind::Elided, BondKind::Single),
            Some((BondKind::Single, BondKind::Single))
        )
    }

    #[test]
    fn elided_up() {
        assert_eq!(
            reconcile(BondKind::Elided, BondKind::Up),
            Some((BondKind::Down, BondKind::Up))
        )
    }

    #[test]
    fn elided_down() {
        assert_eq!(
            reconcile(BondKind::Elided, BondKind::Down),
            Some((BondKind::Up, BondKind::Down))
        )
    }

    #[test]
    fn up_elided() {
        assert_eq!(
            reconcile(BondKind::Up, BondKind::Elided),
            Some((BondKind::Up, BondKind::Down))
        )
    }

    #[test]
    fn down_elided() {
        assert_eq!(
            reconcile(BondKind::Down, BondKind::Elided),
            Some((BondKind::Down, BondKind::Up))
        )
    }

    #[test]
    fn up_down() {
        assert_eq!(
            reconcile(BondKind::Up, BondKind::Down),
            Some((BondKind::Up, BondKind::Down))
        )
    }

    #[test]
    fn down_up() {
        assert_eq!(
            reconcile(BondKind::Down, BondKind::Up),
            Some((BondKind::Down, BondKind::Up))
        )
    }

    #[test]
    fn single_elided() {
        assert_eq!(
            reconcile(BondKind::Single, BondKind::Elided),
            Some((BondKind::Single, BondKind::Single))
        )
    }
}
