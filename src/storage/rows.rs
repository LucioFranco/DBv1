use super::column::Column;
use super::table::Table;
use std::io::{Write, Read, Seek};

/// The Rows struct represents the file level implementation
/// of the table
pub struct Rows<B: Write + Read + Seek> {
    buf: B
}

// TODO: implement insert_row and select_row
impl<B: Write + Read + Seek> Rows<B> {
    pub fn new(buf: B) -> Self {
        Rows {
            buf: buf
        }
    }
}