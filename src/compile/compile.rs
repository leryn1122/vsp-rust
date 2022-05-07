use hashbrown::HashMap;
use crate::cli::cmd;
use crate::cli::cmd::Args;
use crate::compile::lexer::Lexer;
use crate::compile::parser::Parser;

/// Compiler.
pub struct Compiler {
    pub context: Context
}

impl Compiler {

    pub fn new(context: Context) -> Self {
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

        let mut lexer = Lexer::new(&self.context.source);
        lexer.read_as_token_stream();

        let mut parser = Parser::from_token_stream(lexer.token_stream);
        parser.parse();

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
    help_hook: fn(),
}

impl Context {

    fn init(source: String, opts: HashMap<String, Vec<String>>, help_hook: fn()) -> Self {
        Context {
            source,
            opts,
            help_hook
        }
    }

    /// Create a compile context from cli args.
    pub fn from_args(args: Args, help_hook: fn()->()) -> Self {
        Context::init(args.1[1].clone(), cmd::parse_opts(args), help_hook)
    }

}


