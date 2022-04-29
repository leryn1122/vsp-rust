use vsp::cli::cmd::{do_print_version_and_exit, obtain_args};
use vsp::cli::opts::Opt;
use vsp::compile::compile::{Compiler, Context};
use vsp::std::gen::Res;

pub const CMD: &str = "vspc";

/// Entrypoint of vsp compiler
fn main() -> Res<()> {
    let args = obtain_args();
    fast_return(args.1.clone());

    let mut opts = Opt::from_args(args.0, args.1);
    execute(opts);
    Ok(())
}

fn execute(opts: Vec<Opt>) {

    let context = Context::from_opts(opts);
    let compiler = Compiler::new(context);
    compiler.compile("vsp/test.vsp");

    // if opts.verbose {
    //     println!("Verbose mode is on.");
    // }
    // if opts.help {
    //     do_print_help_and_exit();
    // }
    // if opts.version {
    //     do_print_version_and_exit( CMD);
    // }
    //
    // //eprintln!("source file = {}", opts.get_source());
    // //eprintln!("options = {:?}", opts);
    // compile::compile_source(opts.get_source());
}

/// Fast return if opt below was met
/// - <code>--version</code>
/// - <code>--help</code>
fn fast_return(args: Vec<String>) {
    if args.contains(&"--help".to_string()) {
        do_print_help_and_exit();
    }
    if args.contains(&"--version".to_string()) {
        do_print_version_and_exit(CMD);
    }
}

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
