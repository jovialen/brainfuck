//! Lexical analysis

use crate::error::{LexerError, Result};
use itertools::Itertools;

/// Recognized Brainfuck tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Increment the value at the current memory location.
    Increment(u8),
    /// Decrement the value at the current memory location.
    Decrement(u8),
    /// Go to the next byte in memory.
    Next(usize),
    /// Go to the previous byte in memory.
    Prev(usize),
    /// Print the value at the current memory location as a [`char`].
    Print,
    /// Set the value at the current memory location from the standard input.
    Input,
    /// Repeat the block while the current memory location is not zero.
    Closure(Block),
    #[cfg(feature = "debug_token")]
    /// Print the content of the memory as u8.
    Debug,
    #[cfg(feature = "precompiled_patterns")]
    /// A block with a known pre-compiled result.
    Pattern(PreCompiledPattern),
}

#[cfg(feature = "precompiled_patterns")]
/// Pre-compiled patterns of Brainfuck code.
#[derive(Debug, Clone, PartialEq)]
pub enum PreCompiledPattern {
    /// Set the current memory location to zero.
    SetToZero,
    /// Set the destination byte to the current byte multiplied by a constant.
    Multiply {
        /// The offset from the current byte to store the result.
        dest_offset: isize,
        /// The constant to multiply the source byte with.
        factor: u8,
    },
}

/// Vector of [`Token`]s making up a single block of code.
pub type Block = Vec<Token>;

const TOKEN_INCREMENT: char = '+';
const TOKEN_DECREMENT: char = '-';
const TOKEN_NEXT: char = '>';
const TOKEN_PREV: char = '<';
const TOKEN_PRINT: char = '.';
const TOKEN_INPUT: char = ',';
const TOKEN_LOOP_BEGIN: char = '[';
const TOKEN_LOOP_END: char = ']';
#[cfg(feature = "debug_token")]
const TOKEN_DEBUG: char = '#';

/// Parse Brainfuck program.
///
/// This function takes in a source string as an argument and parses it to a
/// block of [`Token`]s, and then optimizes it as much as possible.
///
/// # Arguments
///
/// * `src` - The Brainfuck source to parse.
///
/// # Errors
///
/// If the given source cannot be lexed, a [`LexerError`] will be returned.
///
/// # Examples
///
/// ```
/// use brainfuck_lexer::lexer::lex;
///
/// let src = "++++++++[->++++++++<].".to_string();
/// let code = lex(src);
/// ```
pub fn lex(src: String) -> Result<Block> {
    let mut slice = src
        .chars()
        .into_iter()
        .filter(|ch| !ch.is_whitespace())
        .map(|c| (c, 1))
        .coalesce(|(c, n), (d, m)| {
            if c == d
                && (c == TOKEN_INCREMENT
                    || c == TOKEN_DECREMENT
                    || c == TOKEN_NEXT
                    || c == TOKEN_PREV)
            {
                Ok((c, n + m))
            } else {
                Err(((c, n), (d, m)))
            }
        });

    let res = optimize_block(&tokenize_block(&mut slice, false)?);

    Ok(res)
}

/// Tokenize iterator to Brainfuck block.
fn tokenize_block<T>(iter: &mut T, is_closure: bool) -> Result<Block>
where
    T: Iterator<Item = (char, u32)>,
{
    let mut block = vec![];

    while let Some((ch, count)) = iter.next() {
        let op = match ch {
            TOKEN_INCREMENT => Token::Increment(count as u8),
            TOKEN_DECREMENT => Token::Decrement(count as u8),
            TOKEN_NEXT => Token::Next(count as usize),
            TOKEN_PREV => Token::Prev(count as usize),
            TOKEN_PRINT => Token::Print,
            TOKEN_INPUT => Token::Input,
            TOKEN_LOOP_BEGIN => Token::Closure(tokenize_block(iter, true)?),
            TOKEN_LOOP_END if is_closure => return Ok(block),
            TOKEN_LOOP_END => Err(LexerError::SyntaxError(ch))?,
            #[cfg(feature = "debug_token")]
            TOKEN_DEBUG => Token::Debug,
            #[cfg(feature = "comments")]
            _ => continue,
            #[cfg(not(feature = "comments"))]
            _ => Err(LexerError::SyntaxError(ch))?,
        };

        block.push(op);
    }

    if is_closure {
        Err(LexerError::UnclosedBlock)
    } else {
        Ok(block)
    }
}

fn optimize_block(block: &Block) -> Block {
    block
        .into_iter()
        .map(|token| match token {
            Token::Closure(block) => Token::Closure(optimize_block(block)),
            _ => token.clone(),
        })
        .filter(|token| match token {
            Token::Closure(block) => !block.is_empty(),
            _ => true,
        })
        .map(|token| match token {
            #[cfg(feature = "precompiled_patterns")]
            Token::Closure(block) => match &block[..] {
                &[Token::Decrement(1)] => Token::Pattern(PreCompiledPattern::SetToZero),
                &[Token::Decrement(1), Token::Next(offset), Token::Increment(factor), Token::Prev(rev_offset)] if offset == rev_offset => Token::Pattern(PreCompiledPattern::Multiply { dest_offset: offset as isize, factor: factor }),
                &[Token::Decrement(1), Token::Prev(offset), Token::Increment(factor), Token::Next(rev_offset)] if offset == rev_offset => Token::Pattern(PreCompiledPattern::Multiply { dest_offset: -(offset as isize), factor: factor }),
                &[Token::Next(offset), Token::Increment(factor), Token::Prev(rev_offset), Token::Decrement(1)] if offset == rev_offset => Token::Pattern(PreCompiledPattern::Multiply { dest_offset: offset as isize, factor: factor }),
                &[Token::Prev(offset), Token::Increment(factor), Token::Next(rev_offset), Token::Decrement(1)] if offset == rev_offset => Token::Pattern(PreCompiledPattern::Multiply { dest_offset: -(offset as isize), factor: factor }),
                _ => Token::Closure(block),
            },
            _ => token,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_tokens() {
        let src = "+".to_string();
        let expected = vec![Token::Increment(1)];
        assert_eq!(lex(src), Ok(expected));

        let src = "-".to_string();
        let expected = vec![Token::Decrement(1)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn move_tokens() {
        let src = ">".to_string();
        let expected = vec![Token::Next(1)];
        assert_eq!(lex(src), Ok(expected));

        let src = "<".to_string();
        let expected = vec![Token::Prev(1)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn io_tokens() {
        let src = ".".to_string();
        let expected = vec![Token::Print];
        assert_eq!(lex(src), Ok(expected));

        let src = ",".to_string();
        let expected = vec![Token::Input];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn closure_tokens() {
        let src = "[.]".to_string();
        let expected = vec![Token::Closure(vec![Token::Print])];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn repeatable_tokens() {
        let src = "+++".to_string();
        let expected = vec![Token::Increment(3)];
        assert_eq!(lex(src), Ok(expected));

        let src = "-----".to_string();
        let expected = vec![Token::Decrement(5)];
        assert_eq!(lex(src), Ok(expected));

        let src = ">>".to_string();
        let expected = vec![Token::Next(2)];
        assert_eq!(lex(src), Ok(expected));

        let src = "<<<<<<".to_string();
        let expected = vec![Token::Prev(6)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn non_repeatable_tokens() {
        let src = "...".to_string();
        let expected = vec![Token::Print, Token::Print, Token::Print];
        assert_eq!(lex(src), Ok(expected));

        let src = ",,".to_string();
        let expected = vec![Token::Input, Token::Input];
        assert_eq!(lex(src), Ok(expected));

        let src = "[.][.]".to_string();
        let expected = vec![
            Token::Closure(vec![Token::Print]),
            Token::Closure(vec![Token::Print]),
        ];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn ignore_empty_closures() {
        let src = "[+][][][][+]".to_string();
        let expected = vec![
            Token::Closure(vec![Token::Increment(1)]),
            Token::Closure(vec![Token::Increment(1)]),
        ];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn closure_token_capture() {
        let src = "[+]".to_string();
        let expected = vec![Token::Closure(vec![Token::Increment(1)])];
        assert_eq!(lex(src), Ok(expected));

        let src = "+[+]".to_string();
        let expected = vec![
            Token::Increment(1),
            Token::Closure(vec![Token::Increment(1)]),
        ];
        assert_eq!(lex(src), Ok(expected));

        let src = "[+]+".to_string();
        let expected = vec![
            Token::Closure(vec![Token::Increment(1)]),
            Token::Increment(1),
        ];
        assert_eq!(lex(src), Ok(expected));

        let src = "+[+]+".to_string();
        let expected = vec![
            Token::Increment(1),
            Token::Closure(vec![Token::Increment(1)]),
            Token::Increment(1),
        ];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn closure_errors() {
        let src = "[][".to_string();
        assert_eq!(lex(src), Err(LexerError::UnclosedBlock));

        let src = "[]]".to_string();
        assert_eq!(lex(src), Err(LexerError::SyntaxError(']')));
    }

    #[test]
    fn whitespace() {
        let src = "+ +\n\n\n - -    ".to_string();
        let expected = vec![Token::Increment(2), Token::Decrement(2)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[cfg(feature = "comments")]
    #[test]
    fn comments() {
        let src = "[ This is a comment ]+Inside of the- code".to_string();
        let expected = vec![Token::Increment(1), Token::Decrement(1)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[cfg(feature = "debug_token")]
    #[test]
    fn debug_token() {
        let src = "#".to_string();
        let expected = vec![Token::Debug];
        assert_eq!(lex(src), Ok(expected));
    }

    #[cfg(feature = "precompiled_patterns")]
    mod precompiled_patterns {
        use super::*;

        #[test]
        fn set_to_zero_pattern() {
            let src = "[-]".to_string();
            let expected = vec![Token::Pattern(PreCompiledPattern::SetToZero)];
            assert_eq!(lex(src), Ok(expected));
        }

        #[test]
        fn multiply_pattern() {
            let src = "[->+<]".to_string();
            let expected = vec![Token::Pattern(PreCompiledPattern::Multiply {
                dest_offset: 1,
                factor: 1,
            })];
            assert_eq!(lex(src), Ok(expected));

            let src = "[->>>+<<<]".to_string();
            let expected = vec![Token::Pattern(PreCompiledPattern::Multiply {
                dest_offset: 3,
                factor: 1,
            })];
            assert_eq!(lex(src), Ok(expected));

            let src = "[->++++<]".to_string();
            let expected = vec![Token::Pattern(PreCompiledPattern::Multiply {
                dest_offset: 1,
                factor: 4,
            })];
            assert_eq!(lex(src), Ok(expected));
        }

        #[test]
        fn uneven_offsets() {
            let src = "[->>+<]".to_string();
            let expected = vec![Token::Closure(vec![
                Token::Decrement(1),
                Token::Next(2),
                Token::Increment(1),
                Token::Prev(1),
            ])];
            assert_eq!(lex(src), Ok(expected));
        }
    }
}
