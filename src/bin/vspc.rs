#![allow(dead_code)]

use vsp::cli::cmd;
use vsp::cli::cmd::Args;
use vsp::compile::Compiler;
use vsp::ctx::Context;

extern crate lazy_static;

pub const CMD: &'static str = "vspc";

fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
{} <source> [ [ --options [ params ... ] | --options[=params,...] ] ... ]

  where options may any of:
    --fast        This would ignore all other compile options by default.
    --feature     Enable specified feature.
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
    let args = cmd::obtain_args();
    cmd::fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    vsp::init();
    execute(args);
}

fn execute(args: Args) {
    let source: String = args.1.get(0).unwrap().to_string();
    let context = Context::from_args(args, do_print_help_and_exit);
    let compiler = Compiler::new(context, source);
    compiler.compile();
}
