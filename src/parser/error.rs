use crate::core::Error;
use std::io;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err.to_string())
    }
}
