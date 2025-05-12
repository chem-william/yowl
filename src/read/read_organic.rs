use mendeleev::Element;

use super::{error::ReadError, missing_character::missing_character, scanner::Scanner};
use crate::feature::AtomKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AtomToken {
    Aromatic(Element),
    Aliphatic(Element),
}

/// Try to consume one organic‐atom token (e.g. “c” or “Cl”).
fn next_atom_token(scanner: &mut Scanner) -> Result<Option<AtomToken>, ReadError> {
    match scanner.peek() {
        // aromatic (lowercase single letters)
        Some('b') => {
            scanner.pop();
            Ok(Some(AtomToken::Aromatic(Element::B)))
        }
        Some('c') => {
            scanner.pop();
            Ok(Some(AtomToken::Aromatic(Element::C)))
        }
        Some('n') => {
            scanner.pop();
            Ok(Some(AtomToken::Aromatic(Element::N)))
        }
        Some('o') => {
            scanner.pop();
            Ok(Some(AtomToken::Aromatic(Element::O)))
        }
        Some('p') => {
            scanner.pop();
            Ok(Some(AtomToken::Aromatic(Element::P)))
        }
        Some('s') => {
            scanner.pop();
            Ok(Some(AtomToken::Aromatic(Element::S)))
        }

        // aliphatic: two-char combos first
        Some('B') => {
            scanner.pop();
            if scanner.peek() == Some(&'r') {
                scanner.pop();
                Ok(Some(AtomToken::Aliphatic(Element::Br)))
            } else {
                Ok(Some(AtomToken::Aliphatic(Element::B)))
            }
        }
        Some('C') => {
            scanner.pop();
            if scanner.peek() == Some(&'l') {
                scanner.pop();
                Ok(Some(AtomToken::Aliphatic(Element::Cl)))
            } else {
                Ok(Some(AtomToken::Aliphatic(Element::C)))
            }
        }
        Some('T') => {
            scanner.pop();
            if scanner.peek() == Some(&'s') {
                scanner.pop();
                Ok(Some(AtomToken::Aliphatic(Element::Ts)))
            } else {
                Err(missing_character(scanner))
            }
        }
        Some('A') => {
            scanner.pop();
            if scanner.peek() == Some(&'t') {
                scanner.pop();
                Ok(Some(AtomToken::Aliphatic(Element::At)))
            } else {
                Err(missing_character(scanner))
            }
        }

        // aliphatic: rest of single uppercase letters
        Some('N') => {
            scanner.pop();
            Ok(Some(AtomToken::Aliphatic(Element::N)))
        }
        Some('O') => {
            scanner.pop();
            Ok(Some(AtomToken::Aliphatic(Element::O)))
        }
        Some('P') => {
            scanner.pop();
            Ok(Some(AtomToken::Aliphatic(Element::P)))
        }
        Some('S') => {
            scanner.pop();
            Ok(Some(AtomToken::Aliphatic(Element::S)))
        }
        Some('F') => {
            scanner.pop();
            Ok(Some(AtomToken::Aliphatic(Element::F)))
        }
        Some('I') => {
            scanner.pop();
            Ok(Some(AtomToken::Aliphatic(Element::I)))
        }

        // no match → not an organic atom here
        _ => Ok(None),
    }
}

pub fn read_organic(scanner: &mut Scanner) -> Result<Option<AtomKind>, ReadError> {
    if let Some(token) = next_atom_token(scanner)? {
        // map raw token → domain type
        let kind = match token {
            AtomToken::Aromatic(element) => AtomKind::Aromatic(element),
            AtomToken::Aliphatic(a) => AtomKind::Aliphatic(a),
        };
        Ok(Some(kind))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a_x() {
        let mut scanner = Scanner::new("Ax");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Err(ReadError::Character(1)))
    }

    #[test]
    fn t_x() {
        let mut scanner = Scanner::new("Tx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Err(ReadError::Character(1)))
    }

    #[test]
    fn b_x() {
        let mut scanner = Scanner::new("Bx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Element::B))))
    }

    #[test]
    fn c_x() {
        let mut scanner = Scanner::new("Cx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Element::C))))
    }

    #[test]
    fn scan_aromatics() {
        let aromatic_strings = ["b", "c", "n", "o", "p", "s"];
        let aromatic_results = [
            Element::B,
            Element::C,
            Element::N,
            Element::O,
            Element::P,
            Element::S,
        ];
        for (inp, out) in aromatic_strings.iter().zip(aromatic_results) {
            let mut scanner = Scanner::new(inp);
            let atom = read_organic(&mut scanner);

            assert_eq!(atom, Ok(Some(AtomKind::Aromatic(out))))
        }
    }

    #[test]
    fn chlorine() {
        let mut scanner = Scanner::new("Cl");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Element::Cl))))
    }
}
