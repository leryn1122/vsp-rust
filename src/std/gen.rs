use std::error::Error;
use std::result::Result;

pub type CustomResult<T> = Result<T, Box<dyn Error>>;