use std::fs::File;
use hashbrown::HashMap;
use crate::cli;

use crate::cli::opts::Opt;
use crate::std::gen::Res;

use super::buffer::Buffer;
use super::lexer::LexerPattern;
use super::token::{Token, TokenStream};

pub const CMD: &str = "vspc";

///
/// Compiler.
///
pub struct Compiler {
    context: Context
}

impl Compiler {
    pub fn new(context: Context) -> Compiler {
        Compiler {
            context
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

/// Compile context
///   Provides a context which enables the compiler to obtain miscellaneous environment info.
///
/// 编译上下文
///   提供编译器一个可以获得各种环境信息的上下文环境
pub struct Context {
    opts: HashMap<String, Vec<String>>
}

impl Context {

    fn new() -> Context {
        Context {
            opts: HashMap::new(),
        }
    }

    pub fn from_opts(opts: Vec<Opt>) -> Context {
        let mut context = Context::new();
        opts.iter().for_each( |opt| {
            context.opts.insert(opt.get_name().clone(), opt.get_params().clone());
        });
        std::mem::drop(opts);

        if context.opts.get("verbose").is_some() {
            println!("Verbose mode is on!!");
        }
        context
    }

}


