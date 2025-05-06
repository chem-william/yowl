use std::convert::TryInto;

use logos::Lexer;

use super::{error::ReadError, token::Token};
use crate::feature::Rnum;

pub fn read_rnum(lexer: &mut Lexer<Token>) -> Result<Option<Rnum>, ReadError> {
    let mut digits = String::new();

    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Integer) => digits.push_str(lexer.slice()),
            Ok(Token::Percentage) => {
                for _ in 0..=1 {
                    if let Some(token) = lexer.next() {
                        match token {
                            Ok(Token::Integer) => digits.push_str(lexer.slice()),
                            _ => return Err(ReadError::Character(lexer.span().start)),
                        }
                    }
                }
            }
            _ => return Ok(None),
        }
    }

    let rnum = digits.parse::<u16>().expect("rnum to u16");

    Ok(Some(rnum.try_into().expect("u16 to rnum")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn percent_digit() {
        let mut lexer = Token::lexer("%0");

        assert_eq!(read_rnum(&mut lexer), Err(ReadError::EndOfLine))
    }

    #[test]
    fn zero() {
        let mut lexer = Token::lexer("0");

        assert_eq!(read_rnum(&mut lexer), Ok(Some(Rnum::R0)))
    }

    #[test]
    fn nine() {
        let mut lexer = Token::lexer("9");

        assert_eq!(read_rnum(&mut lexer), Ok(Some(Rnum::R9)))
    }

    #[test]
    fn percent_zero_zero() {
        let mut lexer = Token::lexer("%00");

        assert_eq!(read_rnum(&mut lexer), Ok(Some(Rnum::R0)))
    }

    #[test]
    fn percent_nine_nine() {
        let mut lexer = Token::lexer("%99");

        assert_eq!(read_rnum(&mut lexer), Ok(Some(Rnum::R99)))
    }
}
