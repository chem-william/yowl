use std::str::CharIndices;

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    /// The input SMILES string, assumed to contain only ASCII characters.
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Scanner<'a> {
    /// Create a new Scanner over an ASCII SMILES string. Panic if not ASCII.
    pub fn new(input: &'a str) -> Self {
        // We guarantee ASCII only; rust strings are UTF-8, so check for any byte ≥ 128
        if input.bytes().any(|b| b >= 128) {
            panic!("Scanner only supports ASCII input");
        }
        Scanner {
            input,
            buf: input.as_bytes(),
            pos: 0,
        }
    }

    /// Advance until the next non‐quote byte, returning it as a char, or None if at EOF.
    pub fn pop(&mut self) -> Option<char> {
        while self.pos < self.buf.len() {
            let b = self.buf[self.pos];
            self.pos += 1;
            if b != b'\'' {
                // b < 128, so this is safe
                return Some(b as char);
            }
            // else it was a quote: skip it and continue
        }
        None
    }

    /// Look ahead to the next non‐quote char without consuming. Returns None at EOF.
    pub fn peek(&self) -> Option<char> {
        let mut i = self.pos;
        while i < self.buf.len() {
            let b = self.buf[i];
            if b != b'\'' {
                return Some(b as char);
            }
            i += 1;
        }
        None
    }

    /// The current byte‐index in the original string.
    pub fn cursor(&self) -> usize {
        self.pos
    }

    /// True if we’ve consumed all characters in the string.
    pub fn is_done(&self) -> bool {
        self.peek().is_none()
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_given_empty() {
        let scanner = Scanner::new("");

        assert_eq!(scanner.cursor(), 0);
    }

    #[test]
    fn cursor_given_not_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some('a'));
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn cursor_given_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some('a'));
        assert_eq!(scanner.pop(), Some('b'));
        assert_eq!(scanner.pop(), Some('c'));
        assert_eq!(scanner.cursor(), 3);
    }

    #[test]
    fn is_done_given_done() {
        let scanner = Scanner::new("");

        assert!(scanner.is_done());
    }

    #[test]
    fn is_done_given_not_done() {
        let scanner = Scanner::new("a");

        assert!(!scanner.is_done());
    }

    #[test]
    fn peek_given_not_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some('a'));
        assert_eq!(scanner.peek(), Some('b'));
    }

    #[test]
    fn peek_given_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some('a'));
        assert_eq!(scanner.pop(), Some('b'));
        assert_eq!(scanner.pop(), Some('c'));
        assert_eq!(scanner.peek(), None);
    }

    #[test]
    fn pop_given_not_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some('a'));
    }

    #[test]
    fn pop_given_done() {
        let mut scanner = Scanner::new("a");

        assert_eq!(scanner.pop(), Some('a'));
        assert_eq!(scanner.pop(), None);
    }
}
