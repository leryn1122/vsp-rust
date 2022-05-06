use crate::fs::file::os_platform;
use crate::sys::native;

pub const PID_FILE_PREFIX: &'static str = "vsprefdata_";

pub fn get_pid_file_path() -> String {
    let username = native::get_current_user();
    let dir : String = format!("{}{}{}{}",
                        "/tmp/",
                        os_platform::FILE_SEP,
                        PID_FILE_PREFIX,
                        username);
    dir
}