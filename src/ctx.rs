use std::fmt::{Debug, Formatter};
use hashbrown::HashMap;
use crate::cli::cmd;
use crate::cli::cmd::Args;

/// # Context
///
/// Provides a context which enables the engine to obtain miscellaneous environment
/// info.
/// It would be held by miscellaneous struct to distribute the functions.
///
/// # 上下文
///   提供编译器或虚拟机一个可以获得各种环境信息的上下文环境
pub struct Context {
    /// Original options in CLI
    opts: HashMap<String, Vec<String>>,
    /// Help info hook.
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

impl Debug for Context {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Context");
        ds.field("opts", &self.opts);
        ds.field("help_hook", &self.help_hook);
        ds.finish()
    }
}