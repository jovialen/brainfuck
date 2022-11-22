use crate::error::BrainfuckError;
#[cfg(feature = "precompiled_patterns")]
use brainfuck_lexer::lexer::PreCompiledPattern;
use brainfuck_lexer::{Block, Token};
use std::io::Read;

const HEAP_SIZE: usize = 30_000;

pub fn brainfuck(src: &Block) -> Result<(), BrainfuckError> {
    interpret(src, &mut std::io::stdin(), &mut std::io::stdout())
}

pub fn interpret<I, O>(src: &Block, input: &mut I, out: &mut O) -> Result<(), BrainfuckError>
where
    I: std::io::Read,
    O: std::io::Write,
{
    let mut memory = [0u8; HEAP_SIZE];
    let mut ptr = 0;

    interpret_block(src, &mut memory, &mut ptr, input, out)
}

fn read_u8<I>(input: &mut I) -> std::io::Result<u8>
where
    I: std::io::Read,
{
    input.bytes().next().unwrap_or(Ok(0))
}

fn interpret_block<I, O>(
    block: &Block,
    memory: &mut [u8],
    ptr: &mut usize,
    input: &mut I,
    out: &mut O,
) -> Result<(), BrainfuckError>
where
    I: std::io::Read,
    O: std::io::Write,
{
    for op in block {
        match op {
            Token::Increment(x) => memory[*ptr] = memory[*ptr].wrapping_add(*x),
            Token::Decrement(x) => memory[*ptr] = memory[*ptr].wrapping_sub(*x),
            Token::Next(count) => *ptr = ptr.wrapping_add(*count) % memory.len(),
            Token::Prev(count) => *ptr = ptr.wrapping_sub(*count) % memory.len(),
            Token::Print => write!(out, "{}", memory[*ptr] as char)?,
            Token::Input => memory[*ptr] = read_u8(input)?,
            Token::Closure(block) => {
                while memory[*ptr] != 0 {
                    interpret_block(block, memory, ptr, input, out)?;
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
                    } % memory.len();

                    memory[dest] = memory[*ptr].wrapping_mul(factor);
                    memory[*ptr] = 0;
                }
            },
        }
    }

    Ok(())
}
