#![allow(dead_code)]

use std::fs::read_dir;
use std::path::Path;
use vsp::cli::cmd::{
    fast_return,
    obtain_args
};

pub const CMD: &'static str = "vsps";

/// Help info.
fn do_print_help_and_exit() {
    println!(
//==============================================================================
"\
{} [ options ]

  where options may any of:
    --help        Print help message.
    --version     Print version info.
",
//==============================================================================
    CMD,
    );
    std::process::exit(0);
}

///
/// Entrypoint of vsp process tool.
///
/// The instinct is very simple inspired by JVM.
/// When a runtime program ran through `vspr`, it created a file named with process
/// ID with content of process.
/// The default location is a directory named `vsprefdata` appending the current
/// username under the system temporary directory:
///
/// ```
/// /tmp/vsprefdata_root                   (Linux)
/// %SystemRoot%\TEMP\vsprefdata_root      (Windows)
/// ```
///
fn main() {
    let args = obtain_args();
    fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    execute();
}

fn execute() {
    let dir = vsp::vm::process::get_pid_file_path();
    let user_path = Path::new(&dir);
    for child_dir in read_dir(&user_path) {
        child_dir
            .filter(| f | {
                f.as_ref().unwrap().file_type().unwrap().is_file()
            })
            .for_each(| f | {
                let path = f.unwrap().path();
                println!("{}", path.file_name().unwrap().to_str().unwrap());
            }
        )
    }
}


