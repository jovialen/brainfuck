use std::io::Read;

use clap::Parser;

const HEAP_SIZE: usize = 30_000;

const INCREMENT: char = '+';
const DECREMENT: char = '-';
const NEXT_CELL: char = '>';
const PREV_CELL: char = '<';
const PRINT: char = '.';
const INPUT: char = ',';
const LOOP_BEGIN: char = '[';
const LOOP_END: char = ']';

#[derive(Parser)]
struct Args {
    src: String,
}

#[derive(Debug)]
enum BrainfuckError {
    IOError(std::io::Error),
    UnexpectedEOF,
    ReadError,
    SyntaxError(char),
}

impl From<std::io::Error> for BrainfuckError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
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

fn get_op_from_src_raw(src: &String, pc: usize) -> Option<char> {
    src.chars().nth(pc)
}

fn get_op_from_src(src: &String, pc: usize) -> Result<char, BrainfuckError> {
    get_op_from_src_raw(src, pc).map_or_else(|| Err(BrainfuckError::UnexpectedEOF), |v| Ok(v))
}

fn main() -> Result<(), BrainfuckError> {
    let args = Args::parse();
    let code = get_source_as_str(args.src)?;
    brainfuck(code)
}

fn brainfuck(src: String) -> Result<(), BrainfuckError> {
    let mut pc = 0;
    let mut memory = [0u8; HEAP_SIZE];
    let mut ptr = 0;
    let mut stack = vec![];

    while pc < src.len() {
        let op = get_op_from_src(&src, pc)?;

        match op {
            INCREMENT => memory[ptr] = memory[ptr].wrapping_add(1),
            DECREMENT => memory[ptr] = memory[ptr].wrapping_sub(1),
            NEXT_CELL => ptr = (ptr.wrapping_add(1)) % memory.len(),
            PREV_CELL => ptr = (ptr.wrapping_sub(1)) % memory.len(),
            PRINT => print!("{}", memory[ptr] as char),
            INPUT => {
                memory[ptr] =
                    get_u8_from_stdin().map_or_else(|| Err(BrainfuckError::ReadError), |v| Ok(v))?
            }
            LOOP_BEGIN => {
                if memory[ptr] != 0 {
                    stack.push(pc);
                } else {
                    let mut depth = 1;
                    loop {
                        pc += 1;

                        match get_op_from_src(&src, pc)? {
                            LOOP_BEGIN => depth += 1,
                            LOOP_END => depth -= 1,
                            _ => (),
                        }

                        if depth == 0 {
                            break;
                        }
                    }
                }
            }
            LOOP_END => {
                pc = stack
                    .pop()
                    .map_or_else(|| Err(BrainfuckError::SyntaxError(']')), |v| Ok(v))?
                    - 1;
            }
            #[cfg(not(feature = "comments"))]
            c @ _ => Err(BrainfuckError::SyntaxError(c))?,
            #[cfg(feature = "comments")]
            _ => (),
        }

        pc += 1;
    }

    Ok(())
}
