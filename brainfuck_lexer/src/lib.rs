//! This crate provides a lexer for Brainfuck code.

#![warn(missing_docs)]

pub mod error;
pub mod lexer;

pub use lexer::{lex, Block, Token};
