use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotValidIdentifier(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::NotValidIdentifier(ref ident) => {
                write!(f, "Error: not valid identifier: {}", ident)
            }
        }
    }
}
