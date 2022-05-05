use libc::uid_t;

pub const PID_FILE_PREFIX: &'static str = "vsprefdata_";

unsafe fn get_current_uid() -> uid_t {
    libc::geteuid()
}

pub fn get_current_user() -> &'static str {
    "root"
}