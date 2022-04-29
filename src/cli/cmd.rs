use std::option_env;

/// Return a tuple of <code>argc</code> and <code>argv</code>
///
/// 返回 argc 和 argv 的 tuple
pub fn obtain_args() -> (usize, Vec<String>){
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();
    (argc, argv)
}

pub fn do_print_version_and_exit(cmd: &str) {
    println!(
//==============================================================================
"\
{} version {} (early access)
Repo: {}
",
//==============================================================================
        cmd,
        option_env!("CARGO_PKG_VERSION").unwrap(),
        option_env!("CARGO_PKG_REPOSITORY").unwrap()
    );
    std::process::exit(0);
}