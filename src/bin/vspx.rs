#![allow(dead_code)]

use vsp::cli::cmd;

pub const CMD: &'static str = "vspx";

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

/// Entrypoint of vsp tool for compression and extraction.
///
/// vsp 压缩和解压工具
fn main() {
    let args = cmd::obtain_args();
    cmd::fast_return(args.1.clone(), CMD, do_print_help_and_exit);
}

fn execute() {
    todo!("TODO: vspx")
}
