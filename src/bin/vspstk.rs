use vsp::cli::cmd;

pub const CMD: &'static str = "vspstk";

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

/// Entrypoint of vsp stack tracer.
///
/// vsp 堆栈追踪跟踪器的端点
fn main() {
    let args = cmd::obtain_args();
    cmd::fast_return_without_primary_args(
        args.1.clone(), CMD, do_print_help_and_exit);
    execute();
}

fn execute() {
    todo!("TODO: vspstk")
}
