use std::fmt;
use std::result;
use std::io;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotValidIdentifier(String),
    IoError(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::NotValidIdentifier(ref ident) => {
                write!(f, "Error: not valid identifier: {}", ident)
            }

            &Error::IoError(ref err) => {
                write!(f, "IoError: {}", err)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}
