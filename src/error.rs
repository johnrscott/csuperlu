use std::fmt;

#[derive(Debug)]
pub struct Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured in csuperlu")
    }
}
