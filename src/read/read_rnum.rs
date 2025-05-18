use super::{error::ReadError, missing_character::missing_character, scanner::Scanner};
use crate::feature::Rnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RnumToken {
    /// A single digit, 0–9
    Digit(u8),
    /// A two-digit percent form, e.g. "%12"
    Percent(u8, u8),
}

fn next_rnum_token(scanner: &mut Scanner) -> Result<Option<RnumToken>, ReadError> {
    let result = match scanner.peek() {
        // single digit
        Some('0'..='9') => {
            let c = scanner.pop().unwrap();
            let d = u8::try_from(c.to_digit(10).unwrap()).expect("rnum to u8");

            Ok(Some(RnumToken::Digit(d)))
        }

        // percent-encoded two-digit
        Some('%') => {
            scanner.pop(); // consume '%'

            // first digit
            let c1 = match scanner.peek() {
                Some(next) if next.is_ascii_digit() => next,
                _ => return Err(missing_character(scanner)),
            };
            scanner.pop();

            let d1 = u8::try_from(c1.to_digit(10).unwrap()).expect("rnum as u8");

            // second digit
            let c2 = match scanner.peek() {
                Some(next) if next.is_ascii_digit() => next,
                _ => return Err(missing_character(scanner)),
            };
            scanner.pop();

            let d2 = u8::try_from(c2.to_digit(10).unwrap()).expect("rnum as u8");

            Ok(Some(RnumToken::Percent(d1, d2)))
        }

        // not an r-number here
        _ => Ok(None),
    };

    result
}

pub fn read_rnum(scanner: &mut Scanner) -> Result<Option<Rnum>, ReadError> {
    if let Some(tok) = next_rnum_token(scanner)? {
        let raw = match tok {
            RnumToken::Digit(d) => u16::from(d),
            RnumToken::Percent(d1, d2) => u16::from(d1) * 10 + u16::from(d2),
        };

        let rnum = Rnum::try_from(raw).expect("raw in valid range for Rnum");
        Ok(Some(rnum))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_digit() {
        let mut scanner = Scanner::new("%0");

        assert_eq!(read_rnum(&mut scanner), Err(ReadError::EndOfLine))
    }

    #[test]
    fn zero() {
        let mut scanner = Scanner::new("0");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::new(0))))
    }

    #[test]
    fn nine() {
        let mut scanner = Scanner::new("9");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::new(9))))
    }

    #[test]
    fn percent_zero_zero() {
        let mut scanner = Scanner::new("%00");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::new(0))))
    }

    #[test]
    fn percent_nine_nine() {
        let mut scanner = Scanner::new("%99");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::new(99))))
    }
}
