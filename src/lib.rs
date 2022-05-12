#![allow(dead_code)]

pub mod cli;
pub mod compile;
pub mod fs;
pub mod fstd;
pub mod sys;
pub mod oop;
pub mod vm;
pub mod ctx;

type VspError = Box<dyn std::error::Error>;
type VspResult<T> = Result<T, VspError>;


/// Initialize the logger.
pub fn init() {
    env_logger::init();
}