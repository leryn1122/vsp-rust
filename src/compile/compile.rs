use std::fs::File;
use std::io::Read;

use crate::std::gen::Res;

// use super::ast::ASTNode;
// use super::ast::ASTParser;
use super::buffer::Buffer;
// use super::lexer::Lexer;
// use super::token::Token;

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
        // let mut lexer : Lexer = Lexer::accept(&buffer);
        let mut offset: usize;

        loop {
            offset = file.read(buffer.get()).unwrap();
            // lexer.parse_token();

            eprintln!("{}", offset);
            eprintln!("{:?}", buffer.get());
            if 0 == offset {
                break;
            }
        }
    }
}
