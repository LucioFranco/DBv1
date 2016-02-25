pub mod database;
pub mod table;
pub mod types;

use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    LoadDatabase
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
