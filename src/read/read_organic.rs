use logos::Lexer;

use super::{error::ReadError, token::Token};
use crate::feature::{Aliphatic, Aromatic, AtomKind};

pub fn read_organic(lexer: &mut Lexer<Token>) -> Result<Option<AtomKind>, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::AromaticB) => aromatic(Aromatic::B),
            Ok(Token::AromaticC) => aromatic(Aromatic::C),
            Ok(Token::AromaticN) => aromatic(Aromatic::N),
            Ok(Token::AromaticO) => aromatic(Aromatic::O),
            Ok(Token::AromaticP) => aromatic(Aromatic::P),
            Ok(Token::AromaticS) => aromatic(Aromatic::S),
            Ok(Token::AliphaticAt) => aliphatic(Aliphatic::At),
            Ok(Token::AliphaticBr) => aliphatic(Aliphatic::Br),
            Ok(Token::AliphaticB) => aliphatic(Aliphatic::B),
            Ok(Token::AliphaticC) => aliphatic(Aliphatic::C),
            Ok(Token::AliphaticCl) => aliphatic(Aliphatic::Cl),
            Ok(Token::AliphaticN) => aliphatic(Aliphatic::N),
            Ok(Token::AliphaticO) => aliphatic(Aliphatic::O),
            Ok(Token::AliphaticP) => aliphatic(Aliphatic::P),
            Ok(Token::AliphaticS) => aliphatic(Aliphatic::S),
            Ok(Token::AliphaticF) => aliphatic(Aliphatic::F),
            Ok(Token::AliphaticI) => aliphatic(Aliphatic::I),
            Ok(Token::AliphaticTs) => aliphatic(Aliphatic::Ts),
            _ => todo!(),
        }
    } else {
        Err(ReadError::Character(lexer.span().start))
    }
}

fn aromatic(aromatic: Aromatic) -> Result<Option<AtomKind>, ReadError> {
    Ok(Some(AtomKind::Aromatic(aromatic)))
}

fn aliphatic(aliphatic: Aliphatic) -> Result<Option<AtomKind>, ReadError> {
    Ok(Some(AtomKind::Aliphatic(aliphatic)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;
    use pretty_assertions::assert_eq;

    #[test]
    fn a_x() {
        let mut lexer = Token::lexer(&"Ax");
        let atom = read_organic(&mut lexer);

        assert_eq!(atom, Err(ReadError::Character(1)))
    }

    #[test]
    fn t_x() {
        let mut lexer = Token::lexer(&"Tx");
        let atom = read_organic(&mut lexer);

        assert_eq!(atom, Err(ReadError::Character(1)))
    }

    #[test]
    fn b_x() {
        let mut lexer = Token::lexer("Bx");
        let atom = read_organic(&mut lexer);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::B))))
    }

    #[test]
    fn c_x() {
        let mut lexer = Token::lexer("Cx");
        let atom = read_organic(&mut lexer);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::C))))
    }

    #[test]
    fn aromatic_carbon() {
        let mut lexer = Token::lexer("c");
        let atom = read_organic(&mut lexer);

        assert_eq!(atom, Ok(Some(AtomKind::Aromatic(Aromatic::C))))
    }

    #[test]
    fn chlorine() {
        let mut lexer = Token::lexer("Cl");
        let atom = read_organic(&mut lexer);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::Cl))))
    }
}
