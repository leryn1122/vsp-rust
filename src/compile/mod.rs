use crate::compile::lexer::Lexer;
use crate::compile::optim::OptimizationLevel;
use crate::compile::parser::Parser;
use crate::ctx::Context;
use crate::VspResult;

pub mod error;

mod ast;
mod lexer;
mod optim;
mod parser;
mod token;


/// Compiler.
pub struct Compiler {
    /// Compiler context.
    pub(crate) context: Context,
    /// Entrypoint source to compile.
    pub(crate) source: String,
    /// Level of optimization.
    pub(crate) optimization_level: OptimizationLevel,
}

impl Compiler {

    pub fn new(context: Context, source: String) -> Self {
        Compiler {
            context,
            source,
            optimization_level: OptimizationLevel::default(),
        }
    }

    /// Main entrypoint to compile the source code.
    #[inline(always)]
    pub fn compile(&self) -> VspResult<()> {
        self.compile_with_optimization(
            OptimizationLevel::default()
        )
    }

    /// Compile the source code with the given optimization level.
    #[inline(always)]
    pub fn compile_with_optimization(
        &self,
        optimization_level: OptimizationLevel
    ) -> VspResult<()> {
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

        Ok(())
    }
}

