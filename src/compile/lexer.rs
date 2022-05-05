use std::fs::File;
use std::path::Path;
use crate::fs::buffer::Buffer;
use crate::compile::token::{Token, TokenStream};
use crate::fs::source::SourceFile;

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
#[derive(Debug)]
pub struct Lexer {
    source: SourceFile,
    line: usize,
    buffer: Buffer,
    token_stream: TokenStream,
}

impl Lexer {

    pub fn new(source: &str) -> Self {
        let path = Path::new(source);
        Lexer {
            source: SourceFile::File(path.to_str().unwrap().to_string()),
            line: 0,
            buffer: Buffer::new(),
            token_stream: TokenStream::new(),
        }
    }

    pub fn read_as_token_stream(&mut self) {
        log::trace!("{:?}", self);

        let mut file = File::open(&self.source.get_path()).unwrap();
        let mut buffer: Buffer = Buffer::new();

        let offset: usize;
        offset = buffer.read(&mut file).unwrap();

        let mut pattern = LexerPattern::Trivial;
        let mut str_buf: Vec<char> = Vec::with_capacity(1 << 5);

        let mut token_stream = TokenStream::new();

        // TODO offset should have been replaced by len.
        for i in 0 .. offset {
            let ch : char = buffer.char_at(i);

            match pattern {
                LexerPattern::Trivial => {
                    // Alphabetic or digit ??
                    if ch.is_alphanumeric() {
                        str_buf.push(ch);
                    } else if '"' == ch {
                        pattern = LexerPattern::DoubleQuoted;
                    } else if '\'' == ch {
                        pattern = LexerPattern::SingleQuoted;
                    } else if ch.is_whitespace() {
                        if str_buf.len() > 0 {
                            put_token_stream(&mut token_stream, &mut str_buf);
                            str_buf.clear();
                        }
                    } else if ch.is_ascii_punctuation() {
                        if str_buf.len() > 0 {
                            put_token_stream(&mut token_stream, &mut str_buf);
                            str_buf.clear();
                        }
                        str_buf.push(ch);
                    } else {
                        if str_buf.len() > 0 {
                            put_token_stream(&mut token_stream, &mut str_buf);
                            str_buf.clear();
                        }
                    }
                },
                // Double quoted
                //   All characters are appended, expect another quote.
                LexerPattern::DoubleQuoted => {
                    if '"' == ch {
                        put_token_stream(&mut token_stream, &mut str_buf);
                        pattern = LexerPattern::Trivial;
                    } else {
                        str_buf.push(ch);
                    }
                },
                // Single quoted
                //   All characters are appended, expect another quote.
                LexerPattern::SingleQuoted => {
                    if '\'' == ch {
                        put_token_stream(&mut token_stream, &mut str_buf);
                        pattern = LexerPattern::Trivial;
                    } else {
                        str_buf.push(ch);
                    }
                }
            }

        }
        println!("token={:?}", &token_stream);
        self.token_stream = token_stream;
    }

}

fn put_token_stream(token_stream: &mut TokenStream, str_buf: &mut Vec<char>) {
    if str_buf.len() > 0 {
        let str = str_buf.iter().collect::<String>();
        println!("{}", str);
        token_stream.put(Token::parse_token(&str).unwrap());
        str_buf.clear();
    }
}