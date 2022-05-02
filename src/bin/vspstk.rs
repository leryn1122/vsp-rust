use vsp::cli::cmd::{
    fast_return,
    obtain_args
};

pub const CMD: &str = "vspstk";

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

/// Entrypoint of vsp stack tracer.
///
/// vsp 堆栈追踪跟踪器的端点
fn main() {
    let args = obtain_args();
    fast_return(args.1.clone(), CMD, do_print_help_and_exit);
}
