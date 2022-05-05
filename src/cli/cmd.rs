/// Return a tuple of <code>argc</code> and <code>argv</code>
///
/// 返回 argc 和 argv 的 tuple
pub fn obtain_args() -> (usize, Vec<String>){
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();
    (argc, argv)
}

/// Print the version info.
///
/// ```txt
/// vspc version 0.1.0
/// ```
pub fn do_print_version_and_exit(cmd: &str) {
    println!(
//==============================================================================
"\
{} version {} (early access)",
//==============================================================================
        cmd,
        std::option_env!("CARGO_PKG_VERSION").unwrap(),
    );
    std::process::exit(0);
}

/// Fast return if one of cli options below was met.
///   - <code>--version</code>
///   - <code>--help</code>
///
/// Accept
///   - args
///   - command name
///   - function to print help info.
pub fn fast_return(argv: Vec<String>, cmd: &str, help_hook: fn() -> ()) {
    if argv.len() < 2 {
        help_hook();
    }
    if argv.contains(&"--help".to_string()) {
        help_hook();
    }
    if argv.contains(&"--version".to_string()) {
        do_print_version_and_exit(cmd);
    }
}