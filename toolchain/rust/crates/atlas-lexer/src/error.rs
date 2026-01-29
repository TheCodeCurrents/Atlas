#[derive(Debug, Clone)]
pub enum LexError {
    InvalidCharacter(char, usize),
    InvalidNumber(String, usize),
    UnexpectedEof,
    UnterminatedString(usize),
}
