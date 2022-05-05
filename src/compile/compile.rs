use hashbrown::HashMap;
use crate::cli::cmd;
use crate::cli::cmd::Args;
use crate::compile::lexer::Lexer;

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

    fn init(source: String, opts: HashMap<String, Vec<String>>) -> Self {
        Context {
            source,
            opts,
        }
    }

    /// Create a compile context from cli args.
    pub fn from_args(args: Args) -> Self {
        Context::init(args.1[1].clone(), cmd::parse_opts(args))
    }

}


