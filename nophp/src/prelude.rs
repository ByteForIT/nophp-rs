use std::error::Error;

pub use std::concat as c;
pub use std::format as f;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
