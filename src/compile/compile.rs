use std::fs::File;
use crate::cli::opts::Opt;
use crate::compile::token::{Token, TokenStream};
use crate::std::gen::Res;

use super::buffer::Buffer;
use super::lexer::LexerPattern;

///
/// Compiler.
///
pub struct Compiler {
    opts: Vec<Opt>,
}

impl Compiler {
    pub fn new(opts: Vec<Opt>) -> Compiler {
        Compiler {
            opts
        }
    }

    pub fn compile(&self, src: &str) {
        let mut file = File::open(src).unwrap();

        let mut buffer: Buffer = Buffer::new();

        let mut offset: usize;
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

        println!("{:?}", &token_stream);
    }
}

fn put_token_stream(token_stream: &mut TokenStream, str_buf: &mut Vec<char>) {
    if str_buf.len() > 0 {
        let mut str = str_buf.iter().collect::<String>();
        // println!("lexeme  = {}", str);
        token_stream.put(Token::parse_token(&str));
        str_buf.clear();
    }
}
