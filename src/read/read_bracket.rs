use std::convert::TryInto;

use super::{
    error::ReadError, missing_character, read_charge, read_configuration, read_symbol, scanner::Scanner
};
use crate::feature::{AtomKind, Number, VirtualHydrogen};

pub fn read_bracket(scanner: &mut Scanner) -> Result<Option<AtomKind>, ReadError> {
    if let Some('[') = scanner.peek() {
        scanner.pop();
    } else {
        return Ok(None);
    }

    let isotope = read_isotope(scanner)?;
    let symbol = read_symbol(scanner)?;
    let configuration = read_configuration(scanner)?;
    let hcount = read_hcount(scanner)?;
    let charge = read_charge(scanner)?;
    let map = read_map(scanner)?;

    match scanner.peek() {
        Some(']') => {
            scanner.pop();

            Ok(Some(AtomKind::Bracket {
                isotope,
                symbol,
                configuration,
                hcount,
                charge,
                map,
            }))
        }
        None => Err(ReadError::EndOfLine),
        _ => Err(ReadError::Character(scanner.cursor())),
    }
}

fn read_hcount(scanner: &mut Scanner) -> Result<Option<VirtualHydrogen>, ReadError> {
    match scanner.peek() {
        Some('H') => {
            scanner.pop();

            match scanner.peek() {
                Some('0'..='9') => match scanner.pop() {
                    Some('0') => Ok(Some(VirtualHydrogen::H0)),
                    Some('1') => Ok(Some(VirtualHydrogen::H1)),
                    Some('2') => Ok(Some(VirtualHydrogen::H2)),
                    Some('3') => Ok(Some(VirtualHydrogen::H3)),
                    Some('4') => Ok(Some(VirtualHydrogen::H4)),
                    Some('5') => Ok(Some(VirtualHydrogen::H5)),
                    Some('6') => Ok(Some(VirtualHydrogen::H6)),
                    Some('7') => Ok(Some(VirtualHydrogen::H7)),
                    Some('8') => Ok(Some(VirtualHydrogen::H8)),
                    Some('9') => Ok(Some(VirtualHydrogen::H9)),
                    _ => Ok(Some(VirtualHydrogen::H1)),
                },
                _ => Ok(Some(VirtualHydrogen::H1)),
            }
        }
        _ => Ok(None),
    }
}

fn read_isotope(scanner: &mut Scanner) -> Result<Option<Number>, ReadError> {
    let mut digits = String::new();

    for _ in 0..3 {
        match scanner.peek() {
            Some('0'..='9') => digits.push(*scanner.pop().expect("digit")),
            _ => break,
        }
    }

    if digits.is_empty() {
        Ok(None)
    } else {
        Ok(Some(digits.try_into().expect("number")))
    }
}

fn read_map(scanner: &mut Scanner) -> Result<Option<Number>, ReadError> {
    match scanner.peek() {
        Some(':') => {
            scanner.pop();

            let mut digits = String::new();

            match scanner.pop() {
                Some(next) => {
                    if next.is_ascii_digit() {
                        digits.push(*next);
                    } else {
                        return Err(ReadError::Character(scanner.cursor() - 1));
                    }
                }
                None => return Err(missing_character(scanner)),
            }

            for _ in 0..2 {
                match scanner.peek() {
                    Some('0'..='9') => digits.push(*scanner.pop().expect("digit")),
                    _ => break,
                }
            }

            Ok(Some(digits.try_into().expect("number")))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{BracketAromatic, BracketSymbol, Charge, Configuration};
    use pretty_assertions::assert_eq;
    use std::convert::TryInto;

    #[test]
    fn overflow_map() {
        let mut scanner = Scanner::new("[*:1000]");

        assert_eq!(read_bracket(&mut scanner), Err(ReadError::Character(6)))
    }

    #[test]
    fn overflow_isotope() {
        let mut scanner = Scanner::new("[1000U]");

        assert_eq!(read_bracket(&mut scanner), Err(ReadError::Character(4)))
    }

    #[test]
    fn bracket_invalid() {
        let mut scanner = Scanner::new("[Q]");

        assert_eq!(read_bracket(&mut scanner), Err(ReadError::Character(1)))
    }

    #[test]
    fn no_close() {
        let mut scanner = Scanner::new("[C");

        assert_eq!(read_bracket(&mut scanner), Err(ReadError::EndOfLine))
    }

    #[test]
    fn colon_but_no_map() {
        let mut scanner = Scanner::new("[C:]");

        assert_eq!(read_bracket(&mut scanner), Err(ReadError::Character(3)))
    }

    #[test]
    fn colon_eol() {
        let mut scanner = Scanner::new("[C:");

        assert_eq!(read_bracket(&mut scanner), Err(ReadError::EndOfLine))
    }

    #[test]
    fn no_open() {
        let mut scanner = Scanner::new("?");

        assert_eq!(read_bracket(&mut scanner), Ok(None))
    }

    #[test]
    fn star() {
        let mut scanner = Scanner::new("[*]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: None,
                charge: None,
                map: None
            }))
        )
    }

    #[test]
    fn star_isotope() {
        let mut scanner = Scanner::new("[999*]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: Some(999.try_into().unwrap()),
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: None,
                charge: None,
                map: None
            }))
        )
    }

    #[test]
    fn star_configuration() {
        let mut scanner = Scanner::new("[*@]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: Some(Configuration::TH1),
                hcount: None,
                charge: None,
                map: None
            }))
        )
    }

    #[test]
    fn star_hcount() {
        let mut scanner = Scanner::new("[*H2]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: Some(VirtualHydrogen::H2),
                charge: None,
                map: None
            }))
        )
    }

    #[test]
    fn star_charge() {
        let mut scanner = Scanner::new("[*+]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None
            }))
        )
    }

    #[test]
    fn star_map() {
        let mut scanner = Scanner::new("[*:999]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: None,
                charge: None,
                map: Some(999u16.try_into().unwrap())
            }))
        )
    }

    #[test]
    fn bracket_aromatic_charge() {
        let mut scanner = Scanner::new("[s+]");

        assert_eq!(
            read_bracket(&mut scanner),
            Ok(Some(AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::S),
                configuration: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None
            }))
        )
    }
}
