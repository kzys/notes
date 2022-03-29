use std::error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
