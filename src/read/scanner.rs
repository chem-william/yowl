#[derive(Debug)]
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    pub fn pop(&mut self) -> Option<&char> {
        if let Some(result) = self.characters.get(self.cursor) {
            self.cursor += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Iterator for Scanner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop().copied()
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

        assert_eq!(scanner.pop(), Some(&'a'));
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn cursor_given_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some(&'a'));
        assert_eq!(scanner.pop(), Some(&'b'));
        assert_eq!(scanner.pop(), Some(&'c'));
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

        assert_eq!(scanner.pop(), Some(&'a'));
        assert_eq!(scanner.peek(), Some(&'b'));
    }

    #[test]
    fn peek_given_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some(&'a'));
        assert_eq!(scanner.pop(), Some(&'b'));
        assert_eq!(scanner.pop(), Some(&'c'));
        assert_eq!(scanner.peek(), None);
    }

    #[test]
    fn pop_given_not_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), Some(&'a'));
    }

    #[test]
    fn pop_given_done() {
        let mut scanner = Scanner::new("a");

        assert_eq!(scanner.pop(), Some(&'a'));
        assert_eq!(scanner.pop(), None);
    }
}
