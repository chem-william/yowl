use std::convert::TryInto;

use super::scanner::Scanner;
use crate::feature::Charge;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChargeToken {
    /// +n or -n, where n ∈ 1..=15
    Signed(i8),
    /// “++” → +2
    Plus2,
    /// “--” → −2
    Minus2,
    /// “+” → +1
    Plus1,
    /// “-” → −1
    Minus1,
}

fn next_charge_token(scanner: &mut Scanner) -> Option<ChargeToken> {
    match scanner.peek() {
        Some('+') => {
            scanner.pop();
            // try multi-digit or default
            if let Some(n) = lex_fifteen(scanner) {
                Some(ChargeToken::Signed(n))
            } else if scanner.peek() == Some('+') {
                scanner.pop();
                Some(ChargeToken::Plus2)
            } else {
                Some(ChargeToken::Plus1)
            }
        }
        Some('-') => {
            scanner.pop();
            if let Some(n) = lex_fifteen(scanner) {
                Some(ChargeToken::Signed(-n))
            } else if scanner.peek() == Some('-') {
                scanner.pop();
                Some(ChargeToken::Minus2)
            } else {
                Some(ChargeToken::Minus1)
            }
        }
        _ => None,
    }
}

fn lex_fifteen(scanner: &mut Scanner) -> Option<i8> {
    match scanner.peek() {
        Some('1'..='9') => {
            // first digit
            let c = scanner.pop().unwrap();
            let v = i8::try_from(c.to_digit(10).unwrap()).expect("first digit charge as i8");
            // if that digit was ‘1’, check for 1–5
            if v == 1 {
                if let Some('1'..='5') = scanner.peek() {
                    let c2 = scanner.pop().unwrap();
                    return Some(
                        i8::try_from(c2.to_digit(10).unwrap()).expect("second charge digit as i8")
                            + 10,
                    );
                }
            }
            Some(v)
        }
        _ => None,
    }
}

pub fn read_charge(scanner: &mut Scanner) -> Option<Charge> {
    let tok = next_charge_token(scanner)?;
    let raw = match tok {
        ChargeToken::Signed(n) => n,
        ChargeToken::Plus2 => 2,
        ChargeToken::Minus2 => -2,
        ChargeToken::Plus1 => 1,
        ChargeToken::Minus1 => -1,
    };
    Some(raw.try_into().expect("valid Charge"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn none() {
        let mut scanner = Scanner::new("X");

        assert_eq!(read_charge(&mut scanner), None)
    }

    #[test]
    fn minus_x() {
        let mut scanner = Scanner::new("-X");

        assert_eq!(read_charge(&mut scanner), Charge::new(-1))
    }

    #[test]
    fn minus_2_x() {
        let mut scanner = Scanner::new("-1X");

        assert_eq!(read_charge(&mut scanner), Charge::new(-1))
    }

    #[test]
    fn minus_minus_x() {
        let mut scanner = Scanner::new("--X");

        assert_eq!(read_charge(&mut scanner), Charge::new(-2))
    }

    #[test]
    fn minus_15_x() {
        let mut scanner = Scanner::new("-15X");

        assert_eq!(read_charge(&mut scanner), Charge::new(-15))
    }

    #[test]
    fn plus_x() {
        let mut scanner = Scanner::new("+X");

        assert_eq!(read_charge(&mut scanner), Charge::new(1))
    }

    #[test]
    fn plus_plus_x() {
        let mut scanner = Scanner::new("++X");

        assert_eq!(read_charge(&mut scanner), Charge::new(2))
    }

    #[test]
    fn plus_2_x() {
        let mut scanner = Scanner::new("+2X");

        assert_eq!(read_charge(&mut scanner), Charge::new(2))
    }

    #[test]
    fn plus_15_x() {
        let mut scanner = Scanner::new("+15X");

        assert_eq!(read_charge(&mut scanner), Charge::new(15))
    }
}
