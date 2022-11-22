//! Errors used in the crate.

/// The error type of any lexical analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerError {
    /// Source ended unexpectedly.
    UnexpectedEOF,
    /// Closure with no closing bracket.
    UnclosedBlock,
    /// Syntax error.
    SyntaxError(char),
}

/// Specialized [`Result`] type for lexical analysis.
pub type Result<T> = std::result::Result<T, LexerError>;
