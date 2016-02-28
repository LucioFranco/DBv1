use super::column::Column;
use super::table::Table;

pub struct Rows<'a> {
    table: Table<'a>
}

// TODO: implement insert_row and select_row
impl<'a> Rows<'a> {
    pub fn new<'b>(table: Table<'b>) -> Rows<'b> {
        // TODO: implement new rows
    }
}
