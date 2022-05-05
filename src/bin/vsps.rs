#![allow(dead_code)]

use std::fs::read_dir;
use std::path::Path;
use vsp::cli::cmd::{
    fast_return,
    obtain_args
};
use vsp::fs::file::os_platform;
use vsp::sys::sys;

pub const CMD: &'static str = "vsps";

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
/// vsp 进程工具的端点
fn main() {
    let args = obtain_args();
    fast_return(args.1.clone(), CMD, do_print_help_and_exit);
    execute();
}

fn execute() {
    let username = sys::get_current_user();
    let dir : String = format!("{}{}{}{}",
                               "/tmp/",
                               os_platform::FILE_SEP,
                               sys::PID_FILE_PREFIX,
                               username);
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


