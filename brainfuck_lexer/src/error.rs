#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedEOF,
    UnclosedBlock,
    SyntaxError(char),
}

pub type Result<T> = std::result::Result<T, LexerError>;
