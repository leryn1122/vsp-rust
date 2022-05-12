use crate::compile::lexer::Lexer;
use crate::compile::parser::Parser;
use crate::ctx::Context;

pub mod error;

mod ast;
mod lexer;
mod parser;
mod token;


/// Compiler.
pub struct Compiler {
    pub context: Context,
    pub source: String,
}

impl Compiler {

    pub fn new(context: Context, source: String) -> Self {
        Compiler {
            context,
            source,
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

        let mut lexer = Lexer::new(&self.source);
        lexer.read_as_token_stream();

        let mut parser = Parser::from_token_stream(lexer.token_stream);
        parser.parse();

    }
}

