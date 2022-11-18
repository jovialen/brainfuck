use crate::error::{LexerError, Result};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum Tolken {
    Increment(u8),
    Decrement(u8),
    Next(usize),
    Prev(usize),
    Print,
    Input,
    Closure(Block),
    #[cfg(feature = "debug_tolken")]
    Debug,
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
#[cfg(feature = "debug_tolken")]
const TOLKEN_DEBUG: char = '#';

pub fn lex(src: String) -> Result<Block> {
    let mut slice = src
        .chars()
        .into_iter()
        .filter(|ch| !ch.is_whitespace())
        .map(|c| (c, 1))
        .coalesce(|(c, n), (d, m)| {
            if c == d
                && (c == TOLKEN_INCREMENT
                    || c == TOLKEN_DECREMENT
                    || c == TOLKEN_NEXT
                    || c == TOLKEN_PREV)
            {
                Ok((c, n + m))
            } else {
                Err(((c, n), (d, m)))
            }
        });

    let res = lex_closure(&mut slice, false)?
        .into_iter()
        .filter(|v| match v {
            // Filter out empty closures
            Tolken::Closure(block) => !block.is_empty(),
            _ => true,
        })
        .collect();

    Ok(res)
}

fn lex_closure<T>(iter: &mut T, is_closure: bool) -> Result<Vec<Tolken>>
where
    T: Iterator<Item = (char, u32)>,
{
    let mut block = vec![];

    while let Some((ch, count)) = iter.next() {
        let op = match ch {
            TOLKEN_INCREMENT => Tolken::Increment(count as u8),
            TOLKEN_DECREMENT => Tolken::Decrement(count as u8),
            TOLKEN_NEXT => Tolken::Next(count as usize),
            TOLKEN_PREV => Tolken::Prev(count as usize),
            TOLKEN_PRINT => Tolken::Print,
            TOLKEN_INPUT => Tolken::Input,
            TOLKEN_LOOP_BEGIN => Tolken::Closure(lex_closure(iter, true)?),
            TOLKEN_LOOP_END if is_closure => return Ok(block),
            TOLKEN_LOOP_END => Err(LexerError::SyntaxError(ch))?,
            #[cfg(feature = "debug_tolken")]
            TOLKEN_DEBUG => Tolken::Debug,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_tolkens() {
        let src = "+".to_string();
        let expected = vec![Tolken::Increment(1)];
        assert_eq!(lex(src), Ok(expected));

        let src = "-".to_string();
        let expected = vec![Tolken::Decrement(1)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn move_tolkens() {
        let src = ">".to_string();
        let expected = vec![Tolken::Next(1)];
        assert_eq!(lex(src), Ok(expected));

        let src = "<".to_string();
        let expected = vec![Tolken::Prev(1)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn io_tolkens() {
        let src = ".".to_string();
        let expected = vec![Tolken::Print];
        assert_eq!(lex(src), Ok(expected));

        let src = ",".to_string();
        let expected = vec![Tolken::Input];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn closure_tolkens() {
        let src = "[.]".to_string();
        let expected = vec![Tolken::Closure(vec![Tolken::Print])];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn repeatable_tolkens() {
        let src = "+++".to_string();
        let expected = vec![Tolken::Increment(3)];
        assert_eq!(lex(src), Ok(expected));

        let src = "-----".to_string();
        let expected = vec![Tolken::Decrement(5)];
        assert_eq!(lex(src), Ok(expected));

        let src = ">>".to_string();
        let expected = vec![Tolken::Next(2)];
        assert_eq!(lex(src), Ok(expected));

        let src = "<<<<<<".to_string();
        let expected = vec![Tolken::Prev(6)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn non_repeatable_tolkens() {
        let src = "...".to_string();
        let expected = vec![Tolken::Print, Tolken::Print, Tolken::Print];
        assert_eq!(lex(src), Ok(expected));

        let src = ",,".to_string();
        let expected = vec![Tolken::Input, Tolken::Input];
        assert_eq!(lex(src), Ok(expected));

        let src = "[.][.]".to_string();
        let expected = vec![
            Tolken::Closure(vec![Tolken::Print]),
            Tolken::Closure(vec![Tolken::Print]),
        ];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn ignore_empty_closures() {
        let src = "[+][][][][-]".to_string();
        let expected = vec![
            Tolken::Closure(vec![Tolken::Increment(1)]),
            Tolken::Closure(vec![Tolken::Decrement(1)]),
        ];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn closure_tolken_capture() {
        let src = "[+]".to_string();
        let expected = vec![Tolken::Closure(vec![Tolken::Increment(1)])];
        assert_eq!(lex(src), Ok(expected));

        let src = "+[+]".to_string();
        let expected = vec![
            Tolken::Increment(1),
            Tolken::Closure(vec![Tolken::Increment(1)]),
        ];
        assert_eq!(lex(src), Ok(expected));

        let src = "[+]+".to_string();
        let expected = vec![
            Tolken::Closure(vec![Tolken::Increment(1)]),
            Tolken::Increment(1),
        ];
        assert_eq!(lex(src), Ok(expected));

        let src = "+[+]+".to_string();
        let expected = vec![
            Tolken::Increment(1),
            Tolken::Closure(vec![Tolken::Increment(1)]),
            Tolken::Increment(1),
        ];
        assert_eq!(lex(src), Ok(expected));
    }

    #[test]
    fn whitespace() {
        let src = "+ +\n\n\n - -    ".to_string();
        let expected = vec![Tolken::Increment(2), Tolken::Decrement(2)];
        assert_eq!(lex(src), Ok(expected));
    }

    #[cfg(feature = "comments")]
    #[test]
    fn comments() {
        let src = "[ This is a comment ]+Inside of the- code".to_string();
        let expected = vec![Tolken::Increment(1), Tolken::Decrement(1)];
        assert_eq!(lex(src), Ok(expected));
    }
}
