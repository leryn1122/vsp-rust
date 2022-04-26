use std::fs::File;
use crate::compile::token::Token;

use crate::std::gen::Res;

use super::buffer::Buffer;
use super::lexer::LexerPattern;

pub fn compile_source(src: String) -> Res<()> {
    let compiler: Compiler = Compiler::new();
    compiler.compile(&src);

    Ok(())
}

///
/// Compiler.
///
pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    // pub(crate) fn compile0(&self, entrypoint: &str) {
    //     let mut file = File::open(entrypoint).unwrap();
    //     let mut buffer = [0u8; 1 << 5];
    //     let mut offset: usize;

    //     loop {
    //         offset = file.read(&mut buffer).unwrap();
    //         if 0 == offset {
    //             break;
    //         }
    //         println!("buffer = {:?}", buffer);
    //         println!("offset = {}", offset);
    //         println!("utf8 = {}", String::from_utf8_lossy(&buffer));
    //     }

    //     let mut lex = Lexer::new();
    //     let parser = ASTParser::new();

    //     let tokens: Vec<Token> = lex.lexer_while_scan();
    //     let nodes: Vec<ASTNode> = parser.parse(tokens);
    // }

    pub fn compile(&self, src: &str) {
        let mut file = File::open(src).unwrap();

        let mut buffer: Buffer = Buffer::new();

        let mut offset: usize;
        offset = buffer.read(&mut file).unwrap();

        let mut pattern = LexerPattern::Trivial;
        let mut str_buf: Vec<char> = Vec::with_capacity(1 << 5);

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
                            let mut str = str_buf.iter().collect::<String>();
                            println!("lexeme  = {}", str);
                            println!("{:?}", Token::get_token(&str));
                            str_buf.clear();
                        }
                    } else if ch.is_ascii_punctuation() {
                        if str_buf.len() > 0 {
                            let mut str = str_buf.iter().collect::<String>();
                            println!("lexeme  = {}", str);
                            println!("{:?}", Token::get_token(&str));
                            str_buf.clear();
                        }
                        str_buf.push(ch);
                    } else {
                        if str_buf.len() > 0 {
                            let mut str = str_buf.iter().collect::<String>();
                            println!("lexeme  = {}", str);
                            println!("{:?}", Token::get_token(&str));
                            str_buf.clear();
                        }
                    }
                },
                // Double quoted
                //   All characters are appended, expect another quote.
                LexerPattern::DoubleQuoted => {
                    if '"' == ch {
                        let mut str = str_buf.iter().collect::<String>();
                        println!("literal = {}", str);
                        println!("{:?}", Token::get_token(&str));
                        str_buf.clear();
                        pattern = LexerPattern::Trivial;
                    } else {
                        str_buf.push(ch);
                    }
                },
                // Single quoted
                //   All characters are appended, expect another quote.
                LexerPattern::SingleQuoted => {
                    if '\'' == ch {
                        let mut str = str_buf.iter().collect::<String>();
                        println!("lexeme  = {}", str);
                        println!("{:?}", Token::get_token(&str));
                        str_buf.clear();
                        pattern = LexerPattern::Trivial;
                    } else {
                        str_buf.push(ch);
                    }
                }
            }
        }
    }
}
