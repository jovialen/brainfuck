use crate::error::BrainfuckError;
#[cfg(feature = "precompiled_patterns")]
use brainfuck_lexer::lexer::PreCompiledPattern;
use brainfuck_lexer::{Block, Token};
use std::io::Read;

const HEAP_SIZE: usize = 30_000;

pub fn brainfuck(src: Block, out: &mut dyn std::io::Write) -> Result<(), BrainfuckError> {
    let mut memory = [0u8; HEAP_SIZE];
    let mut ptr = 0;

    interpret_block(&src, &mut memory, &mut ptr, out)
}

fn get_u8_from_stdin() -> Option<u8> {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
}

fn interpret_block(
    block: &Block,
    memory: &mut [u8; HEAP_SIZE],
    ptr: &mut usize,
    out: &mut dyn std::io::Write,
) -> Result<(), BrainfuckError> {
    for op in block {
        match op {
            Token::Increment(x) => memory[*ptr] = memory[*ptr].wrapping_add(*x),
            Token::Decrement(x) => memory[*ptr] = memory[*ptr].wrapping_sub(*x),
            Token::Next(count) => *ptr = ptr.wrapping_add(*count) % memory.len(),
            Token::Prev(count) => *ptr = ptr.wrapping_sub(*count) % memory.len(),
            Token::Print => write!(out, "{}", memory[*ptr] as char)?,
            Token::Input => {
                memory[*ptr] =
                    get_u8_from_stdin().map_or_else(|| Err(BrainfuckError::ReadError), |v| Ok(v))?
            }
            Token::Closure(block) => {
                while memory[*ptr] != 0 {
                    interpret_block(block, memory, ptr, out)?;
                }
            }
            #[cfg(feature = "debug_token")]
            Token::Debug => writeln!(
                out,
                "\n{:?}",
                memory
                    .iter()
                    .scan(0, |state, &cell| {
                        if cell == 0 {
                            *state += 1;
                        } else {
                            *state = 0;
                        }

                        if *state > 3 {
                            None
                        } else {
                            Some(cell)
                        }
                    })
                    .collect::<Vec<_>>()
            )?,
            #[cfg(feature = "precompiled_patterns")]
            Token::Pattern(pattern) => match *pattern {
                PreCompiledPattern::SetToZero => memory[*ptr] = 0,
                PreCompiledPattern::Multiply {
                    dest_offset,
                    factor,
                } => {
                    let dest = if dest_offset > 0 {
                        ptr.wrapping_add(dest_offset as usize)
                    } else {
                        ptr.wrapping_sub(dest_offset.abs() as usize)
                    } % HEAP_SIZE;

                    memory[dest] = memory[*ptr].wrapping_mul(factor);
                    memory[*ptr] = 0;
                }
            },
        }
    }

    Ok(())
}
