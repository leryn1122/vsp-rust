#![allow(dead_code)]

pub mod cli;
pub mod compile;
pub mod fs;
pub mod fstd;
pub mod sys;
pub mod oop;
pub mod vm;
pub mod ctx;

/// Public error type.
pub type VspError = Box<dyn std::error::Error>;
/// Public result type.
pub type VspResult<T> = Result<T, VspError>;

pub type SmartVec<T> = smallvec::SmallVec<[T; 1<<2]>;
// pub type SmartString = smartString::SmartString;

/// Initialize the logger.
pub fn init() {
    env_logger::init();
}