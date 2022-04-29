use std::option_env;

pub fn obtain_args() -> (usize, Vec<String>){
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();
    (argc, argv)
}

pub fn do_print_version_and_exit(cmd: &str) {
    println!(
//==============================================================================
"\
{} ver.{} (early access)
Repo: {}
",
//==============================================================================
        cmd,
        option_env!("CARGO_PKG_VERSION").unwrap(),
        option_env!("CARGO_PKG_REPOSITORY").unwrap()
    );
    std::process::exit(0);
}