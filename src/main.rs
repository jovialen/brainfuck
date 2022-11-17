use std::io::Read;

use clap::Parser;

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

fn get_source_as_str(src: String) -> Result<String, std::io::Error> {
    let path = std::path::Path::new(&src);

    if path.is_file() {
        std::fs::read_to_string(path.to_path_buf())
    } else {
        Ok(src)
    }
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

    let code = get_source_as_str(args.src)?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let mut pc = 0;
    let mut memory = [0u8; 30_000];
    let mut ptr = 0;
    let mut stack = vec![];

    while pc < code.len() {
        let op = code
            .chars()
            .nth(pc)
            .map_or_else(|| Err(BrainfuckError::UnexpectedEOF), |v| Ok(v))?;

        match op {
            '+' => memory[ptr] = memory[ptr].wrapping_add(1),
            '-' => memory[ptr] = memory[ptr].wrapping_sub(1),
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '.' => print!("{}", memory[ptr] as char),
            ',' => {
                memory[ptr] =
                    get_u8_from_stdin().map_or_else(|| Err(BrainfuckError::ReadError), |v| Ok(v))?
            }
            '[' => {
                if memory[ptr] != 0 {
                    stack.push(pc);
                } else {
                    let mut depth = 1;
                    loop {
                        pc += 1;

                        match code
                            .chars()
                            .nth(pc)
                            .map_or_else(|| Err(BrainfuckError::UnexpectedEOF), |v| Ok(v))?
                        {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => (),
                        }

                        if depth == 0 {
                            break;
                        }
                    }
                }
            }
            ']' => {
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
