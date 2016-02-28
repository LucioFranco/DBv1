// TODO: Remove this and fix the dead code warnings
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod database;
pub mod table;
pub mod rows;
pub mod column;
pub mod types;

use std::io;
use bincode::rustc_serialize::{DecodingError};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    BinDe(DecodingError),
    LoadDatabase,
    LoadTable
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<DecodingError> for Error {
    fn from(err: DecodingError) -> Self {
        Error::BinDe(err)
    }
}
