use crate::compile::lexer::Lexer;
use crate::compile::optim::OptimizationLevel;
use crate::compile::parser::Parser;
use crate::ctx::Context;
use crate::fs::source::SourceFile;
use crate::VspResult;

pub mod error;

mod ast;
mod lexer;
mod optim;
mod parser;
mod token;
mod tokenizer;


/// Compiler.
pub struct Compiler {
    /// Compiler context.
    pub(crate) context: Context,
    /// Level of optimization.
    pub(crate) optimization_level: OptimizationLevel,
}

impl Compiler {

    pub fn new(context: Context) -> Self {
        Compiler {
            context,
            optimization_level: OptimizationLevel::default(),
        }
    }

    /// Main entrypoint to compile the source code.
    #[inline(always)]
    pub fn compile(&mut self, source: &str) -> VspResult<()> {
        self.compile_with_optimization(
            source,
            OptimizationLevel::default()
        )
    }

    /// Compile the source code with the given optimization level.
    #[inline(always)]
    pub fn compile_with_optimization(
        &mut self,
        source: &str,
        optimization_level: OptimizationLevel
    ) -> VspResult<()> {
        let source_file = SourceFile::from_path(source).unwrap();
        source_file.read();

        self.optimization_level = optimization_level;

        let mut lexer = Lexer::new(&source);
        lexer.read_as_token_stream();

        let mut parser = Parser::from_token_stream(lexer.token_stream);
        parser.parse();

        Ok(())
    }
}

