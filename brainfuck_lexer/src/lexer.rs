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

const INCREMENT: char = '+';
const DECREMENT: char = '-';
const NEXT: char = '>';
const PREV: char = '<';
const PRINT: char = '.';
const INPUT: char = ',';
const LOOP_BEGIN: char = '[';
const LOOP_END: char = ']';

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
            INCREMENT => Tolken::Increment,
            DECREMENT => Tolken::Decrement,
            NEXT => Tolken::Next,
            PREV => Tolken::Prev,
            PRINT => Tolken::Print,
            INPUT => Tolken::Input,
            LOOP_BEGIN => Tolken::Closure(lex_closure(iter, true)?),
            LOOP_END if is_closure => return Ok(block),
            LOOP_END => Err(LexerError::SyntaxError(ch))?,
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
