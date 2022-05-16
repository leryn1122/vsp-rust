#![allow(dead_code)]

use vsp::cli::cmd;

pub const CMD: &'static str = "vsprepl";

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

/// Entrypoint of vsp repl.
///
/// vsp 交互式解释器
fn main() {
    let args = cmd::obtain_args();
    cmd::fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    execute();
}

#[allow(unused_labels)]
fn execute() {

    'main_loop: loop {
        println!("todo");
        break;
    }

    println!("Bye!!");
}
