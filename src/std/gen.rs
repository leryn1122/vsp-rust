use std::error::Error;
use std::result::Result;

pub type Res<T> = Result<T, Box<dyn Error>>;