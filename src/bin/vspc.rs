use vsp::cli::cmd::{do_print_version_and_exit, obtain_args};
use vsp::cli::opts::Opt;
use vsp::cli::option::*;
use vsp::compile::compile::Compiler;
use vsp::std::gen::Res;

const CMD: &str = "vspc";

/// Entrypoint of vsp compiler
fn main() -> Res<()> {
    let args = obtain_args();
    let mut opts = Opt::from_args(args.0, args.1);
    exec_command(opts);
    Ok(())
}

fn exec_command(opts: Vec<Opt>) {

    let compiler = Compiler::new(opts);

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

fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
{} [ [options] ... ] <source>
where options may one or any of:
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
