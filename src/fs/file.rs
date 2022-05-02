use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use zip::read::ZipFile;
use zip::ZipArchive;

use crate::fs::file::SourceFile::{Dir, TextFile, Zip};

pub type ZipRef = ZipArchive<File>;

#[derive(PartialEq)]
pub enum SourceFile {
    Dir(String),
    TextFile(String),
    // Zip(ZipRef),
    Zip
}

impl SourceFile {

    pub fn from_path(path: &str) -> SourceFile {
        let fname = Path::new(&*path);
        if fname.is_dir() {
            Dir(fname.as_os_str().to_str().unwrap().to_string())
        } else {
            let mut file = File::open(&fname).unwrap();
            let mut magic_number = [0u8; 4];
            file.read(&mut magic_number).unwrap();
            if magic_number.starts_with(&[80u8, 75u8, 3u8, 4u8]) {
                // Zip(ZipArchive::new(file).unwrap())
                Zip
            } else {
                TextFile(fname.as_os_str().to_str().unwrap().to_string())
            }
        }
    }

    pub fn read(&self) {
    }

}

