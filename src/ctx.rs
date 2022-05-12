use hashbrown::HashMap;
use crate::cli::cmd;
use crate::cli::cmd::Args;

/// # Context
///
/// Provides a context which enables the engine to obtain miscellaneous environment
/// info.
///
/// 上下文
///   提供编译器一个可以获得各种环境信息的上下文环境
pub struct Context {
    opts: HashMap<String, Vec<String>>,
    help_hook: fn(),
}

impl Context {

    fn init(opts: HashMap<String, Vec<String>>, help_hook: fn()) -> Self {
        Context {
            opts,
            help_hook,
        }
    }

    /// Create a compile context from cli args.
    pub fn from_args(args: Args, help_hook: fn()->()) -> Self {
        Context::init(cmd::parse_opts(args), help_hook)
    }

}