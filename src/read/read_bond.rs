use super::scanner::Scanner;
use crate::feature::BondKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BondToken {
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down,
    Elided,
}

fn next_bond_token(scanner: &mut Scanner) -> BondToken {
    let tok = match scanner.peek() {
        Some('-') => BondToken::Single,
        Some('=') => BondToken::Double,
        Some('#') => BondToken::Triple,
        Some('$') => BondToken::Quadruple,
        Some(':') => BondToken::Aromatic,
        Some('/') => BondToken::Up,
        Some('\\') => BondToken::Down,
        _ => BondToken::Elided,
    };
    // only consume if it wasn’t elided
    if tok != BondToken::Elided {
        scanner.pop();
    }
    tok
}

pub fn read_bond(scanner: &mut Scanner) -> BondKind {
    match next_bond_token(scanner) {
        BondToken::Single => BondKind::Single,
        BondToken::Double => BondKind::Double,
        BondToken::Triple => BondKind::Triple,
        BondToken::Quadruple => BondKind::Quadruple,
        BondToken::Aromatic => BondKind::Aromatic,
        BondToken::Up => BondKind::Up,
        BondToken::Down => BondKind::Down,
        BondToken::Elided => BondKind::Elided,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elided() {
        let mut scanner = Scanner::new("X");

        assert_eq!(read_bond(&mut scanner), BondKind::Elided)
    }

    #[test]
    fn single() {
        let mut scanner = Scanner::new("-");

        assert_eq!(read_bond(&mut scanner), BondKind::Single);
    }

    #[test]
    fn double() {
        let mut scanner = Scanner::new("=");

        assert_eq!(read_bond(&mut scanner), BondKind::Double);
    }

    #[test]
    fn triple() {
        let mut scanner = Scanner::new("#");

        assert_eq!(read_bond(&mut scanner), BondKind::Triple);
    }

    #[test]
    fn quadruple() {
        let mut scanner = Scanner::new("$");

        assert_eq!(read_bond(&mut scanner), BondKind::Quadruple);
    }

    #[test]
    fn aromatic() {
        let mut scanner = Scanner::new(":");

        assert_eq!(read_bond(&mut scanner), BondKind::Aromatic);
    }

    #[test]
    fn up() {
        let mut scanner = Scanner::new("/");

        assert_eq!(read_bond(&mut scanner), BondKind::Up);
    }

    #[test]
    fn down() {
        let mut scanner = Scanner::new("\\");

        assert_eq!(read_bond(&mut scanner), BondKind::Down);
    }
}
