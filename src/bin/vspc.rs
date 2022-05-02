use vsp::cli::cmd::{
    fast_return,
    obtain_args
};
use vsp::compile::compile::{Compiler, Context};

pub const CMD: &str = "vspc";

fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
{} <source> [ options [ params ... ] ... ]

  where options may any of:
    --feature
    --help        Print help message.
    --profile     Activate the specified profile to enable those APIs.
    --version     Print version info.
",
//==============================================================================
    CMD,
    );
    std::process::exit(0);
}

/// Entrypoint of vsp compiler
///
/// vsp 编译器的端点
fn main() {
    let args = obtain_args();
    fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    execute(args.0, args.1);
}

fn execute(argc: usize, argv: Vec<String>) {
    let context = Context::from_args(argc, argv);
    let compiler = Compiler::new(context);
    compiler.compile();
}
