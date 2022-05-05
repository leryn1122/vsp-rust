use std::fs::File;
use zip::read::ZipFile;
use zip::ZipArchive;

#[cfg(unix)]
pub mod os_platform {
    pub const FILE_SEP: &str = "/";
    pub const PATH_SEP: &str = ":";
}

#[cfg(windows)]
pub mod os_platform {
    pub const FILE_SEP: &str = "\\";
    pub const PATH_SEP: &str = ";";
}

pub type ZipRef = ZipArchive<File>;

