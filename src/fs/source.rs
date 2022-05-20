use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use lazy_static::lazy_static;
use zip::ZipArchive;

use crate::fs::file::{os_platform, ZipRef};
use crate::VspResult;

lazy_static! {
    static ref SFM: Mutex<SourceFileManager> = Mutex::new(SourceFileManager::new());
}

pub fn init() {
    lazy_static::initialize(&SFM);
}

/// Enum for source file mode. That could be a text file / directory or a zip file.
#[derive(Debug)]
pub enum SourceFile {
    Dir(String),
    File(String),
    Zip(String, ZipRef),
}

/// Magic number of ZIP `8075 0304`
const MAGIC_NUMBER_OF_ZIP: &'static [u8; 4] = &[80u8, 75u8, 3u8, 4u8];

impl SourceFile {

    /// Determines whether a source file is a text file / directory or a zip file.
    /// A zip file must begin with `8075 0304`, magic number of zip. Only condition
    /// is the correct magic number
    #[must_use]
    pub fn from_path(path: &str) -> VspResult<SourceFile> {
        match Path::new(&*path).canonicalize() {
            Err(e) => {
                eprintln!("Failed to recognize the path of source code: {}\n{}",
                          path,
                          e);
                std::process::exit(1);
            },
            Ok(filename) => {
                if filename.is_dir() {
                    return Ok(SourceFile::Dir(stringify_path_buf(&filename)))
                }

                let mut file = File::open(&filename).unwrap_or_else(
                    | e | {
                        eprintln!("Failed to open the source code: {}\n{}",
                                  stringify_path_buf(&filename),
                                  e);
                        std::process::exit(1);
                    }
                );
                let mut magic_number = [0u8; 4];
                file.read(&mut magic_number).unwrap();
                if magic_number.starts_with(MAGIC_NUMBER_OF_ZIP) {
                    return Ok(SourceFile::Zip(stringify_path_buf(&filename),
                                       ZipArchive::new(file).unwrap()));
                } else {
                    return Ok(SourceFile::File(stringify_path_buf(&filename)));
                }
            }
        };
    }

    pub fn read(&self) {
        match self {
            SourceFile::Dir(_) => {
                todo!("TODO: Read zip file")
            }
            SourceFile::File(filename) => {
                // Already validated in ctxor.
                let mut file = File::open(&filename).unwrap();
                read_file_with_validation(&mut file);
            }
            SourceFile::Zip(_, _) => {
                todo!("TODO: Read zip file")
            }
        }
    }

    pub fn get_path(&self) -> &str {
        match self {
            SourceFile::Dir(path) => path,
            SourceFile::File(path) => path,
            SourceFile::Zip(path, _) => path,
        }
    }

}

/// Source file statistics information, to determine whether the source file has
/// been changed.
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

/// Stringify the `std::path::PathBuf`.
pub(crate) fn stringify_path_buf(p: &PathBuf) -> String {
    p.as_os_str().to_str().unwrap().to_string()
}

fn read_file_with_validation(file: &mut File) {
    println!("{}", file.metadata().unwrap().is_file());
}