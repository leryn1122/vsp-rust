use vsp::std::gen::Res;

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
fn main() -> Res<()> {
    do_print_help_and_exit();
    Ok(())
}
