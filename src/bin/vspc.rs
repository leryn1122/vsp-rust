#![allow(dead_code)]

use vsp::cli;
use vsp::cli::cmd::{
    Args,
    fast_return,
    obtain_args
};
use vsp::compile::compile::{Compiler, Context};

extern crate lazy_static;

pub const CMD: &'static str = "vspc";

fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
{} <source> [ [ --options [ params ... ] | --options[=params,...] ] ... ]

  where options may any of:
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
    let args = obtain_args();
    fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    cli::init::init();
    execute(args);
}

fn execute(args: Args) {
    let context = Context::from_args(args);
    let compiler = Compiler::new(context);
    compiler.compile();
}
