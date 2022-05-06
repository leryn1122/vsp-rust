use libc::uid_t;

unsafe fn get_current_uid() -> uid_t {
    libc::geteuid()
}

/// Wrapper of native `geteuid()`.
/// TODO
pub fn get_current_user() -> &'static str {
    "root"
}