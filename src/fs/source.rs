use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;
use zip::ZipArchive;

use crate::fs::file::{os_platform, ZipRef};

lazy_static! {
    static ref SFM: Mutex<SourceFileManager> = Mutex::new(SourceFileManager::new());
}

pub fn init() {
    lazy_static::initialize(&SFM);
}

#[derive(Debug)]
pub enum SourceFile {
    Dir(String),
    File(String),
    Zip(String, ZipRef),
}

/// Magic number of ZIP `8075 0304`
const MAGIC_NUMBER_OF_ZIP: [u8; 4] = [80u8, 75u8, 3u8, 4u8];

impl SourceFile {

    /// Determines whether a source file is a normal file / directory or a zip file.
    /// A zip file must begin with magic number of zip.
    pub fn from_path(path: &str) -> Result<SourceFile, std::io::Error> {
        let fname = Path::new(&*path);
        return if fname.is_dir() {
            Ok(SourceFile::Dir(fname.as_os_str().to_str().unwrap().to_string()))
        } else {
            let mut file = File::open(&fname).unwrap();
            let mut magic_number = [0u8; 4];
            file.read(&mut magic_number).unwrap();
            if magic_number.starts_with(&MAGIC_NUMBER_OF_ZIP) {
                Ok(SourceFile::Zip(fname.as_os_str().to_str().unwrap().to_string(),
                                   ZipArchive::new(file).unwrap()))
            } else {
                Ok(SourceFile::File(fname.as_os_str().to_str().unwrap().to_string()))
            }
        }
    }

    pub fn read(&self) {
    }

    pub fn get_path(&self) -> &str {
        match self {
            SourceFile::Dir(path) => path,
            SourceFile::File(path) => path,
            SourceFile::Zip(path, _) => path,
        }
    }

}

#[derive(Debug)]
pub struct SourceFileInfo {
    pub last_modified: String,
    pub size: usize,
    pub checksum: String,
}

struct SourceFileManager {
    path: Vec<SourceFile>
}

impl SourceFileManager {

    fn new() -> Self {
        Self {
            path: vec![],
        }
    }

    fn add_source_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        self.path.push(SourceFile::from_path(path).unwrap());
        Ok(())
    }

    fn add_source_files(&mut self, paths: &str) {
        paths.split(os_platform::PATH_SEP).for_each(
            | path | match self.add_source_file(path) {
                Err(e) => eprintln!("{}", e),
                _ => (),
            }
        );
    }

    pub fn size(&self) -> usize {
        self.path.len()
    }
}

