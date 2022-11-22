//! Errors used in the crate
//!
use brainfuck_lexer::error::LexerError;

/// The error type of any interpreter error.
#[derive(Debug)]
pub enum BrainfuckError {
    /// Any IO error.
    IOError(std::io::Error),
    /// Error with lexical analysis.
    ParserError(LexerError),
}

impl From<std::io::Error> for BrainfuckError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<LexerError> for BrainfuckError {
    fn from(e: LexerError) -> Self {
        Self::ParserError(e)
    }
}
