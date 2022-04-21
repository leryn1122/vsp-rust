use std::env;

use vsp::cli::option::*;
use vsp::cli::cmd::do_print_version_and_exit;
use vsp::compile::compile;
use vsp::std::gen::Res;

///
/// Entrypoint of vsp compiler
///
fn main() -> Res<()> {
    let argv : Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    let opts: Opts = parse_config(argc, argv);
    exec_command(opts);

    Ok(())
}

fn exec_command(opts: Opts) {

    if opts.verbose {
        println!("Verbose mode is on.");
    }
    if opts.help {
        do_print_help_and_exit();
    }
    if opts.version {
        do_print_version_and_exit("vspc");
    }

    //eprintln!("source file = {}", opts.get_source());
    //eprintln!("options = {:?}", opts);
    compile::compile_source(opts.get_source());
}

fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
vspc [ options ... ] <source>
    --help        Print help info.
    --version     Print version info.

Repo: {}
",
//==============================================================================
        option_env!("CARGO_PKG_REPOSITORY").unwrap()
    );
    std::process::exit(0);
}
