use super::{
    error::ReadError, missing_character, read_charge, read_configuration, read_symbol,
    scanner::Scanner,
};
use crate::feature::{AtomKind, BracketSymbol, Charge, Configuration, VirtualHydrogen};

/// The raw tokens you can see inside “[ … ]”
#[derive(Debug, PartialEq, Eq)]
enum BracketToken {
    /// An isotope number, like “13”
    Isotope(u16),
    /// An element symbol, e.g. “C”, “Cl”
    Symbol(BracketSymbol),
    /// Configuration flags, e.g. “@” or “@@”
    Configuration(Configuration),
    /// Hydrogen count, H0…H9 (default H1 if no digit)
    HCount(VirtualHydrogen),
    /// Charge, like “+”, “-”, “++”, “-2”
    Charge(Charge),
    /// Mapping number, like “:1”
    Map(u16),
    /// The closing bracket ‘]’
    Close,
}

fn lex_bracket_contents(scanner: &mut Scanner) -> Result<Vec<BracketToken>, ReadError> {
    let mut tokens = Vec::new();

    // 1. Isotope (0–3 digits)
    if let Some(num) = read_isotope(scanner) {
        tokens.push(BracketToken::Isotope(num));
    }

    // 2. Symbol (1–2 letters)
    let sym = read_symbol(scanner)?;
    tokens.push(BracketToken::Symbol(sym));

    // 3. Configuration (@ or @@)
    if let Some(cfg) = read_configuration(scanner) {
        tokens.push(BracketToken::Configuration(cfg));
    }

    // 4. H count
    if let Some(h) = read_hcount(scanner) {
        tokens.push(BracketToken::HCount(h));
    }

    // 5. Charge
    if let Some(ch) = read_charge(scanner) {
        tokens.push(BracketToken::Charge(ch));
    }

    // 6. Map
    if let Some(m) = read_map(scanner)? {
        tokens.push(BracketToken::Map(m));
    }

    // 7. Expect closing bracket
    match scanner.peek() {
        Some(']') => {
            scanner.pop();
            tokens.push(BracketToken::Close);
            Ok(tokens)
        }
        None => Err(ReadError::EndOfLine),
        _ => Err(ReadError::Character(scanner.cursor())),
    }
}

pub fn read_bracket(scanner: &mut Scanner) -> Result<Option<AtomKind>, ReadError> {
    // Only lex if we see “[”
    if scanner.peek() != Some(&'[') {
        return Ok(None);
    }
    scanner.pop();

    // Lex all pieces
    let tokens = lex_bracket_contents(scanner)?;

    // Now parse tokens into a Bracket struct
    let mut iter = tokens.into_iter().peekable();

    // 1. Isotope?
    let isotope = if let Some(BracketToken::Isotope(_)) = iter.peek() {
        // Now that peek() told us it's correct, safely consume it:
        if let BracketToken::Isotope(n) = iter.next().unwrap() {
            Some(n)
        } else {
            unreachable!() // peek/next disagree
        }
    } else {
        None
    };

    // 2. Symbol (must exist)
    let symbol = match iter.next() {
        Some(BracketToken::Symbol(s)) => s,
        _ => unreachable!("symbol lexing always returns one"),
    };

    // 3. Configuration?
    let configuration = if let Some(BracketToken::Configuration(_)) = iter.peek() {
        if let BracketToken::Configuration(c) = iter.next().unwrap() {
            Some(c)
        } else {
            unreachable!()
        }
    } else {
        None
    };

    // 4. H count?
    let hcount = if let Some(BracketToken::HCount(h)) = iter.peek() {
        if let BracketToken::HCount(h) = iter.next().unwrap() {
            Some(h)
        } else {
            unreachable!()
        }
    } else {
        None
    };

    // 5. Charge?
    let charge = if let Some(BracketToken::Charge(_)) = iter.peek() {
        if let BracketToken::Charge(c) = iter.next().unwrap() {
            Some(c)
        } else {
            unreachable!()
        }
    } else {
        None
    };

    // 6. Map?
    let map = if let Some(BracketToken::Map(_)) = iter.peek() {
        if let BracketToken::Map(m) = iter.next().unwrap() {
            Some(m)
        } else {
            unreachable!()
        }
    } else {
        None
    };

    // 7. Consume final Close
    if let Some(BracketToken::Close) = iter.next() {
        Ok(Some(AtomKind::Bracket {
            isotope,
            symbol,
            configuration,
            hcount,
            charge,
            map,
        }))
    } else {
        Err(ReadError::EndOfLine)
    }
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
    use crate::feature::{BracketAromatic, BracketSymbol, Charge, Configuration};
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
