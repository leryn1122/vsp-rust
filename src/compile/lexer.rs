use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::compile::buffer::Buffer;
use crate::compile::token::TokenStream;

/// Pattern / mode for lexer:
///   - Trivial for normal;
///   - SingleQuoted for character;
///   - DoubleQuoted for literal string.
#[derive(PartialEq)]
pub enum LexerPattern {
    Trivial,
    SingleQuoted,
    DoubleQuoted,
}

/// Lexer for a single file.
pub struct Lexer {
    // path: Box<Path>,
    line: usize,
    // reader: Box<dyn BufRead>,
    buffer: Buffer,
    token_stream: TokenStream,
}

impl Lexer {

    pub fn new(source: &str) -> Self {
        let path = Path::new(source);
        let file = File::options().create(false).read(true).open(path).unwrap();

        Lexer {
            // path: (*path).clone(),
            line: 0,
            buffer: Buffer::new(),
            token_stream: TokenStream::new(),
        }
    }
}