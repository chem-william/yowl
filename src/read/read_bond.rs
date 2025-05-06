use logos::Lexer;

use super::token::Token;
use crate::feature::BondKind;

pub fn read_bond(lexer: &mut Lexer<Token>) -> BondKind {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::SingleBond) => BondKind::Single,
            Ok(Token::DoubleBond) => BondKind::Double,
            Ok(Token::TripleBond) => BondKind::Triple,
            Ok(Token::QuadrupleBond) => BondKind::Quadruple,
            Ok(Token::AromaticBond) => BondKind::Aromatic,
            Ok(Token::UpBond) => BondKind::Up,
            Ok(Token::DownBond) => BondKind::Down,
            _ => todo!(),
        }
    } else {
        BondKind::Elided
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn elided() {
        let mut lexer = Token::lexer("X");

        assert_eq!(read_bond(&mut lexer), BondKind::Elided)
    }

    #[test]
    fn single() {
        let mut lexer = Token::lexer("-");

        assert_eq!(read_bond(&mut lexer), BondKind::Single);
    }

    #[test]
    fn double() {
        let mut lexer = Token::lexer("=");

        assert_eq!(read_bond(&mut lexer), BondKind::Double);
    }

    #[test]
    fn triple() {
        let mut lexer = Token::lexer("#");

        assert_eq!(read_bond(&mut lexer), BondKind::Triple);
    }

    #[test]
    fn quadruple() {
        let mut lexer = Token::lexer("$");

        assert_eq!(read_bond(&mut lexer), BondKind::Quadruple);
    }

    #[test]
    fn aromatic() {
        let mut lexer = Token::lexer(":");

        assert_eq!(read_bond(&mut lexer), BondKind::Aromatic);
    }

    #[test]
    fn up() {
        let mut lexer = Token::lexer("/");

        assert_eq!(read_bond(&mut lexer), BondKind::Up);
    }

    #[test]
    fn down() {
        let mut lexer = Token::lexer("\\");

        assert_eq!(read_bond(&mut lexer), BondKind::Down);
    }
}
