use mendeleev::Isotope;

use super::{
    error::ReadError, missing_character, read_charge, read_configuration, read_symbol,
    scanner::Scanner,
};
use crate::feature::{AtomKind, BracketSymbol, VirtualHydrogen};

fn lex_bracket_contents(scanner: &mut Scanner) -> Result<AtomKind, ReadError> {
    // (We know the '[' was already popped by read_bracket)
    // Read isotope *before* symbol so we can match element+mass in one go:
    let iso_num_opt = read_isotope(scanner);

    // Read the symbol
    let symbol = read_symbol(scanner)?;

    // Build optional `Isotope` only if `symbol` is an [`Element`]
    let isotope = if let BracketSymbol::Element(el) = symbol {
        iso_num_opt.and_then(|mass| {
            Isotope::list()
                .iter()
                .find(|iso| iso.element() == el && iso.mass_number() == mass as u32)
                .cloned()
        })
    } else {
        None
    };

    // 5. The rest are all optional
    let configuration = read_configuration(scanner);
    let hcount = read_hcount(scanner);
    let charge = read_charge(scanner);
    let map = read_map(scanner)?;

    match scanner.peek() {
        Some(']') => {
            scanner.pop();
        }
        _ => return Err(missing_character(scanner)),
    }

    Ok(AtomKind::Bracket {
        isotope,
        symbol,
        configuration,
        hcount,
        charge,
        map,
    })
}

pub fn read_bracket(scanner: &mut Scanner) -> Result<Option<AtomKind>, ReadError> {
    // Only lex if we see “[”
    match scanner.peek() {
        Some('[') => {
            scanner.pop();
        }
        _ => return Ok(None),
    }

    // Lex all pieces
    let bracket = lex_bracket_contents(scanner)?;

    Ok(Some(bracket))
}

fn read_hcount(scanner: &mut Scanner) -> Option<VirtualHydrogen> {
    match scanner.peek() {
        Some('H') => {
            scanner.pop();

            match scanner.peek() {
                Some('0'..='9') => match scanner.pop() {
                    Some('0') => Some(VirtualHydrogen::H0),
                    Some('1') => Some(VirtualHydrogen::H1),
                    Some('2') => Some(VirtualHydrogen::H2),
                    Some('3') => Some(VirtualHydrogen::H3),
                    Some('4') => Some(VirtualHydrogen::H4),
                    Some('5') => Some(VirtualHydrogen::H5),
                    Some('6') => Some(VirtualHydrogen::H6),
                    Some('7') => Some(VirtualHydrogen::H7),
                    Some('8') => Some(VirtualHydrogen::H8),
                    Some('9') => Some(VirtualHydrogen::H9),
                    _ => Some(VirtualHydrogen::H1),
                },
                _ => Some(VirtualHydrogen::H1),
            }
        }
        _ => None,
    }
}

fn read_isotope(scanner: &mut Scanner) -> Option<u16> {
    let mut digits = String::new();

    for _ in 0..3 {
        match scanner.peek() {
            Some('0'..='9') => digits.push(*scanner.pop().expect("digit")),
            _ => break,
        }
    }

    if digits.is_empty() {
        None
    } else {
        Some(digits.parse::<u16>().expect("number"))
    }
}

fn read_map(scanner: &mut Scanner) -> Result<Option<u16>, ReadError> {
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

            Ok(Some(digits.parse::<u16>().expect("number")))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{Aromatic, BracketSymbol, Charge, Configuration};
    use pretty_assertions::assert_eq;

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
                map: Some(999)
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
                symbol: BracketSymbol::Aromatic(Aromatic::S),
                configuration: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None
            }))
        )
    }
}
