use super::column::Column;
use super::table::Table;
use super::Error;
use std::io::{Write, Read, Seek, SeekFrom};

/// The Rows struct represents the file level implementation
/// of the table
pub struct Rows<B: Write + Read + Seek> {
    buf: B,
    
    table_header_offset: u32,
    columns_offset: u32,
}

impl<B: Write + Read + Seek> Rows<B> {
    pub fn new(buf: B, table_header_offset: u32, columns_offset: u32) -> Self {
        Rows {
            buf: buf,
            table_header_offset: table_header_offset,
            columns_offset: columns_offset
        }
    }

    pub fn insert_data(&mut self, data: &[u8]) -> Result<usize, Error> {
        info!("inserting row");
        try!(self.buf.seek(SeekFrom::End(0)));
        self.write_bytes(data)
    }

    fn write_bytes(&mut self, data: &[u8]) -> Result<usize, Error> {
        match self.buf.write_all(data) {
            Ok(_) => Ok(data.len()),
            Err(e) => Err(Error::Io(e))
        }
    }

    // TODO: write select_data function
}

