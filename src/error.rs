use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GpxError {
    what: String,
}

impl GpxError {
    pub fn new(msg: &str) -> Self {
        Self {
            what: String::from(msg),
        }
    }
}

impl Error for GpxError {}

impl fmt::Display for GpxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GpxError: {}", self.what)
    }
}
