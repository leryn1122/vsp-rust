use std::fs::File;
use std::path::Path;
use crate::fs::buffer::Buffer;
use crate::compile::token::{
    parse_token,
    TokenStream
};
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
pub struct Lexer {
    source: SourceFile,
    line: usize,
    buffer: Buffer,
    status: Status,
    pub token_stream: TokenStream,
}

impl Lexer {

    pub fn new(source: &str) -> Self {
        let path = Path::new(source);
        Self {
            source: SourceFile::File(path.to_str().unwrap().to_string()),
            line: 0,
            buffer: Buffer::new(),
            status: Status::new(),
            token_stream: TokenStream::new(),
        }
    }

    pub fn read_as_token_stream(&mut self) {
        let mut file = File::open(&self.source.get_path()).unwrap();
        let mut buffer: Buffer = Buffer::new();

        let offset: usize;
        offset = buffer.read(&mut file).unwrap();

        let mut str_buf: Vec<char> = Vec::with_capacity(1 << 5);

        let mut token_stream = TokenStream::new();

        // TODO offset should have been replaced by len.
        for i in 0 .. offset + 1 {
            let ch : char = buffer.char_at(i);

            match self.get_pattern() {
                LexerPattern::Trivial => {
                    // Alphabetic or digit ??
                    if ch.is_alphanumeric() {
                        if str_buf.last().is_some() && str_buf.last().unwrap().is_ascii_punctuation() {
                            put_token_stream(&mut token_stream, &mut str_buf);
                            str_buf.push(ch);
                        } else {
                            str_buf.push(ch);
                        }
                    } else if '"' == ch {
                        self.set_pattern(LexerPattern::DoubleQuoted);
                    } else if '\'' == ch {
                        self.set_pattern(LexerPattern::SingleQuoted);
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
                        self.set_pattern(LexerPattern::Trivial);
                    } else {
                        str_buf.push(ch);
                    }
                },
                // Single quoted
                //   All characters are appended, expect another quote.
                LexerPattern::SingleQuoted => {
                    if '\'' == ch {
                        put_token_stream(&mut token_stream, &mut str_buf);
                        self.set_pattern(LexerPattern::Trivial);
                    } else {
                        str_buf.push(ch);
                    }
                }
            }

        }
        log::warn!("token_stream = {:?}", &token_stream);
        self.token_stream = token_stream;
    }

    fn get_pattern(&self) -> &LexerPattern {
        &self.status.pattern
    }

    fn set_pattern(&mut self, pattern: LexerPattern) {
        self.status.pattern = pattern
    }

}

/// Status of lexer.
struct Status {
    pub pattern: LexerPattern,
}

impl Status {
    pub fn new() -> Self {
        Self {
            pattern: LexerPattern::Trivial,
        }
    }
}

fn put_token_stream(token_stream: &mut TokenStream, str_buf: &mut Vec<char>) {
    if str_buf.len() > 0 {
        let str = str_buf.iter().collect::<String>();
        token_stream.put(parse_token(&str).unwrap());
        str_buf.clear();
    }
}