#[derive(Debug)]
pub struct Scanner<'a> {
    input: &'a str,
    /// Byte‐offset into `input`; always lands on a valid `char` boundary.
    cursor: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner { input, cursor: 0 }
    }

    /// Return the current byte‐offset (0..=input.len()).
    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    /// True if we’ve consumed all bytes in the string.
    pub fn is_done(&self) -> bool {
        self.cursor >= self.input.len()
    }

    /// Look at the next `char` without advancing.
    pub fn peek(&self) -> Option<char> {
        // If `cursor` is already at or past the end, there is no next char.
        let slice = &self.input[self.cursor..];
        slice.chars().next()
    }

    /// Consume and return the next `char`, advancing `cursor` by that char’s UTF‑8 length.
    pub fn pop(&mut self) -> Option<char> {
        // Use `peek()` to see if there’s a next char.
        if let Some(ch) = self.peek() {
            // Advance by the number of bytes this `char` takes.
            self.cursor += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
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
