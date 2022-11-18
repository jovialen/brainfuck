use brainfuck_lexer::error::LexerError;

#[derive(Debug)]
pub enum BrainfuckError {
    IOError(std::io::Error),
    ParserError(LexerError),
    ReadError,
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
