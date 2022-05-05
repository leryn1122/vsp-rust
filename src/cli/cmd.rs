use hashbrown::HashMap;
use crate::std::substring::Substring;

/// Tuple of argc and argv.
pub type Args = (usize, Vec<String>);

/// Return a tuple of <code>argc</code> and <code>argv</code>
///
/// 返回 argc 和 argv 的 tuple
pub fn obtain_args() -> Args {
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

///
/// ```bash
/// cmd --opts_1 params_1 \
///     --opts_2 params_2 params_3 \
///     --opts_3=params_4,params_5
/// =>
/// {"opts_1": ["params_1"], "opts_2": ["params_2", "params_3"], "opts_3": ["params_4", "params_5"]}
/// ```
///
/// Return a hashmap containing opts and parameters.
pub fn parse_opts(args: Args) -> HashMap<String, Vec<String>> {
    let mut opts = HashMap::new();
    let mut opt: String = String::from("");
    let mut params: Vec<String> = Vec::new();
    for _i in 2 .. args.0 {
        let segment = args.1[_i].to_string();
        if segment.starts_with("--") {
            if opt.len() > 0 {
                opts.insert(opt, params.clone());
                params = Vec::new();
            }
            let _j = segment.find('=');
            opt = segment.substring(2, _j.unwrap_or(segment.len())).to_string();
            if _j.is_some() {
                segment.substring(_j.unwrap() + 1, segment.len()).split(',')
                    .for_each(| s | {
                        params.push(s.to_string())
                    })
            }
        } else {
            params.push(segment);
        }
    }
    opts.insert(opt, params.clone());
    opts
}