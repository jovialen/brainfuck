use crate::error::BrainfuckError;
use brainfuck_lexer::{Block, Tolken};
use std::io::Read;

const HEAP_SIZE: usize = 30_000;

pub fn brainfuck(src: Block) -> Result<(), BrainfuckError> {
    let mut memory = [0u8; HEAP_SIZE];
    let mut ptr = 0;

    interpret_block(&src, &mut memory, &mut ptr)
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
) -> Result<(), BrainfuckError> {
    for op in block {
        match op {
            Tolken::Increment(x) => memory[*ptr] = memory[*ptr].wrapping_add(*x),
            Tolken::Decrement(x) => memory[*ptr] = memory[*ptr].wrapping_sub(*x),
            Tolken::Next(count) => *ptr = ptr.wrapping_add(*count) % memory.len(),
            Tolken::Prev(count) => *ptr = ptr.wrapping_sub(*count) % memory.len(),
            Tolken::Print => print!("{}", memory[*ptr] as char),
            Tolken::Input => {
                memory[*ptr] =
                    get_u8_from_stdin().map_or_else(|| Err(BrainfuckError::ReadError), |v| Ok(v))?
            }
            Tolken::Closure(block) => {
                while memory[*ptr] != 0 {
                    interpret_block(block, memory, ptr)?;
                }
            }
        }
    }

    Ok(())
}
