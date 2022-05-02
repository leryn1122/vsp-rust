use std::fs::File;

use hashbrown::HashMap;

use crate::cli;
use crate::cli::opts::Opt;
use crate::fs::file::{SourceFile};
use crate::std::gen::Res;
use crate::std::substring::Substring;

use super::buffer::Buffer;
use super::lexer::LexerPattern;
use super::token::{Token, TokenStream};

pub const CMD: &str = "vspc";

/// Compiler.
pub struct Compiler {
    context: Context
}

impl Compiler {

    pub fn new(context: Context) -> Compiler {
        Compiler {
            context
        }
    }

    pub fn compile(&self) {

        // let mut f_src = SourceFile::from_path(&self.context.source);
        // match f_src {
        //     SourceFile::Dir(d) => {
        //         println!("dir = {}", d);
        //         todo!("dir");
        //     },
        //     SourceFile::TextFile(f) => {
        //         println!("file = {}", f);
        //         todo!("file");
        //     },
        //     SourceFile::Zip => {
        //         todo!("zip");
        //     }
        // }


        let mut file = File::open(&self.context.source).unwrap();
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
    source: String,
    opts: HashMap<String, Vec<String>>,
}

impl Context {

    fn init(source: String, opts: HashMap<String, Vec<String>>) -> Context {
        Context {
            source,
            opts,
        }
    }

    pub fn from_args(argc: usize, argv: Vec<String>) -> Context {
        let mut opts = HashMap::new();
        let mut opt: String = String::from("");
        let mut params: Vec<String> = Vec::new();
        for _i in 2 .. argc {
            let segment = argv[_i].to_string();
            if segment.starts_with("--") {
                if opt.len() > 0 {
                    opts.insert(opt, params.clone());
                    params = Vec::new();
                }
                opt = segment.substring(2, segment.len()).to_string();
            } else {
                params.push(segment);
            }
        }
        opts.insert(opt, params.clone());

        Context::init(argv[1].clone(), opts)
    }

}


