mod cli;
mod error;
mod interpreter;

use brainfuck_lexer::lex;
use clap::Parser;
use error::BrainfuckError;
use interpreter::brainfuck;

fn get_raw_source_as_str(src: String) -> std::io::Result<String> {
    let path = std::path::Path::new(&src);

    if path.is_file() {
        std::fs::read_to_string(path.to_path_buf())
    } else {
        Ok(src)
    }
}

fn get_source_as_str(src: String) -> std::io::Result<String> {
    Ok(get_raw_source_as_str(src)?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect())
}

fn main() -> Result<(), BrainfuckError> {
    let args = cli::Args::parse();
    let src = get_source_as_str(args.src)?;
    let code = lex(src)?;
    brainfuck(code)
}
