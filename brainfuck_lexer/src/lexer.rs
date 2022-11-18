use crate::error::{LexerError, Result};

#[derive(Debug, Clone)]
pub enum Tolken {
    Increment,
    Decrement,
    Next,
    Prev,
    Print,
    Input,
    Closure(Block),
}

pub type Block = Vec<Tolken>;

const TOLKEN_INCREMENT: char = '+';
const TOLKEN_DECREMENT: char = '-';
const TOLKEN_NEXT: char = '>';
const TOLKEN_PREV: char = '<';
const TOLKEN_PRINT: char = '.';
const TOLKEN_INPUT: char = ',';
const TOLKEN_LOOP_BEGIN: char = '[';
const TOLKEN_LOOP_END: char = ']';

pub fn lex(src: String) -> Result<Block> {
    let mut slice = src.chars().into_iter();
    lex_closure(&mut slice, false)
}

fn lex_closure<T>(iter: &mut T, is_closure: bool) -> Result<Vec<Tolken>>
where
    T: Iterator<Item = char>,
{
    let mut block = vec![];

    while let Some(ch) = iter.next() {
        let op = match ch {
            TOLKEN_INCREMENT => Tolken::Increment,
            TOLKEN_DECREMENT => Tolken::Decrement,
            TOLKEN_NEXT => Tolken::Next,
            TOLKEN_PREV => Tolken::Prev,
            TOLKEN_PRINT => Tolken::Print,
            TOLKEN_INPUT => Tolken::Input,
            TOLKEN_LOOP_BEGIN => Tolken::Closure(lex_closure(iter, true)?),
            TOLKEN_LOOP_END if is_closure => return Ok(block),
            TOLKEN_LOOP_END => Err(LexerError::SyntaxError(ch))?,
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
