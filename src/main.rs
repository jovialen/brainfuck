use std::io::Read;

use brainfuck_lexer::{error::LexerError, lex, Block, Tolken};
use clap::Parser;

const HEAP_SIZE: usize = 30_000;

#[derive(Parser)]
struct Args {
    src: String,
}

#[derive(Debug)]
enum BrainfuckError {
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

fn get_raw_source_as_str(src: String) -> Result<String, std::io::Error> {
    let path = std::path::Path::new(&src);

    if path.is_file() {
        std::fs::read_to_string(path.to_path_buf())
    } else {
        Ok(src)
    }
}

fn get_source_as_str(src: String) -> Result<String, std::io::Error> {
    Ok(get_raw_source_as_str(src)?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>())
}

fn get_u8_from_stdin() -> Option<u8> {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
}

fn main() -> Result<(), BrainfuckError> {
    let args = Args::parse();
    let src = get_source_as_str(args.src)?;
    let code = lex(src)?;
    brainfuck(code)
}

fn brainfuck(src: Block) -> Result<(), BrainfuckError> {
    let mut memory = [0u8; HEAP_SIZE];
    let mut ptr = 0;

    interpret_block(&src, &mut memory, &mut ptr)
}

fn interpret_block(
    block: &Block,
    memory: &mut [u8; HEAP_SIZE],
    ptr: &mut usize,
) -> Result<(), BrainfuckError> {
    for op in block {
        match op {
            Tolken::Increment => memory[*ptr] = memory[*ptr].wrapping_add(1),
            Tolken::Decrement => memory[*ptr] = memory[*ptr].wrapping_sub(1),
            Tolken::Next => *ptr = ptr.wrapping_add(1) % memory.len(),
            Tolken::Prev => *ptr = ptr.wrapping_sub(1) % memory.len(),
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
