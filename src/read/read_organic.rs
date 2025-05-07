use super::{error::ReadError, missing_character::missing_character, scanner::Scanner};
use crate::feature::{Aliphatic, Aromatic, AtomKind};

pub fn read_organic(scanner: &mut Scanner) -> Result<Option<AtomKind>, ReadError> {
    let atom = match scanner.peek() {
        Some('b') => aromatic(Aromatic::B, scanner),
        Some('c') => aromatic(Aromatic::C, scanner),
        Some('n') => aromatic(Aromatic::N, scanner),
        Some('o') => aromatic(Aromatic::O, scanner),
        Some('p') => aromatic(Aromatic::P, scanner),
        Some('s') => aromatic(Aromatic::S, scanner),
        Some('A') => {
            scanner.pop();

            if scanner.peek() == Some(&'t') {
                aliphatic(Aliphatic::At, scanner)
            } else {
                return Err(missing_character(scanner));
            }
        }
        Some('B') => {
            scanner.pop();

            if scanner.peek() == Some(&'r') {
                aliphatic(Aliphatic::Br, scanner)
            } else {
                Some(AtomKind::Aliphatic(Aliphatic::B))
            }
        }
        Some('C') => {
            scanner.pop();

            if scanner.peek() == Some(&'l') {
                aliphatic(Aliphatic::Cl, scanner)
            } else {
                Some(AtomKind::Aliphatic(Aliphatic::C))
            }
        }
        Some('N') => aliphatic(Aliphatic::N, scanner),
        Some('O') => aliphatic(Aliphatic::O, scanner),
        Some('P') => aliphatic(Aliphatic::P, scanner),
        Some('S') => aliphatic(Aliphatic::S, scanner),
        Some('F') => aliphatic(Aliphatic::F, scanner),
        Some('I') => aliphatic(Aliphatic::I, scanner),
        Some('T') => {
            scanner.pop();

            if scanner.peek() == Some(&'s') {
                aliphatic(Aliphatic::Ts, scanner)
            } else {
                return Err(missing_character(scanner));
            }
        }

        // anything else: not an organic atom
        _ => None,
    };

    Ok(atom)
}

fn aromatic(aromatic: Aromatic, scanner: &mut Scanner) -> Option<AtomKind> {
    scanner.pop();

    Some(AtomKind::Aromatic(aromatic))
}

fn aliphatic(aliphatic: Aliphatic, scanner: &mut Scanner) -> Option<AtomKind> {
    scanner.pop();

    Some(AtomKind::Aliphatic(aliphatic))
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

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::B))))
    }

    #[test]
    fn c_x() {
        let mut scanner = Scanner::new("Cx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::C))))
    }

    #[test]
    fn scan_aromatics() {
        let aromatic_strings = ["b", "c", "n", "o", "p", "s"];
        let aromatic_results = [
            Aromatic::B,
            Aromatic::C,
            Aromatic::N,
            Aromatic::O,
            Aromatic::P,
            Aromatic::S,
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

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::Cl))))
    }
}
