use super::{error::ReadError, scanner::Scanner};

pub fn missing_character(scanner: &Scanner) -> ReadError {
    if scanner.is_done() {
        ReadError::EndOfLine
    } else {
        ReadError::Character(scanner.cursor())
    }
}
