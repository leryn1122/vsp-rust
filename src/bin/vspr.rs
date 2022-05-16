#![allow(dead_code)]

use vsp::cli::cmd;

pub const CMD: &'static str = "vspr";

fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
{} <source> [ options [ params ... ] ... ]

  where options may any of:
    --help        Print help message.
    --version     Print version info.
",
//==============================================================================
    CMD,
    );
    std::process::exit(0);
}

/// Entrypoint of vsp runtime
///
/// vsp 运行时的端点
fn main() {
    let args = cmd::obtain_args();
    cmd::fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    execute();
}

fn execute() {
    todo!("TODO: vspr")
}
