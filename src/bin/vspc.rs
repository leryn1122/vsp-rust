use vsp::cli::cmd::{
    do_print_version_and_exit,
    fast_return,
    obtain_args
};
use vsp::cli::opts::Opt;
use vsp::compile::compile::{Compiler, Context};
use vsp::std::gen::Res;

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
fn main() -> Res<()> {
    let args = obtain_args();
    fast_return(args.1.clone(), CMD, do_print_help_and_exit);

    let mut opts = Opt::from_args(args.0, args.1);
    execute(opts);
    Ok(())
}

fn execute(opts: Vec<Opt>) {

    let context = Context::from_opts(opts);
    let compiler = Compiler::new(context);
    compiler.compile("vsp/test.vsp");
}
