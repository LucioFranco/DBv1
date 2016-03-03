use std::fs::{File, OpenOptions, metadata};
use std::path::{Path, PathBuf};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, encode_into, decode_from};

use super::Error;
use super::database::Database;
use super::column::Column;
use super::rows::Rows;

use super::super::identifier::Identifier;

pub struct Table<'a> {
    database: &'a Database,
    name: Identifier,
    meta_data: TableMetadata
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
struct TableMetadata {
    engine_id: u8,
    columns: Vec<Column>
}

/// Table representation
impl<'a> Table<'a> {
    /// Create table in database
    pub fn create<'b>(name: &str, engine_id: u8, columns: Vec<Column>, db: &'b Database) -> Result<Table<'b>, Error> {
        info!("creating table: {}", name);

        // TODO: verify if name is a valid identifier

        let metadata = TableMetadata {
            engine_id: engine_id,
            columns: columns
        };

        let mut buf = try!(File::create(db.get_path().clone().join(&name).with_extension("tbl")));

        encode_into(&metadata, &mut buf, SizeLimit::Infinite).unwrap();

        Ok(Table {
            database: &db,
            name: try!(Identifier::new(name)),
            meta_data: metadata.clone()
        })
    }

    /// Load table from database
    pub fn load<'b>(name: &str, db: &'b Database) -> Result<Table<'b>, Error> {
        info!("loading table: {}", &name);
        let path = db.get_path().clone().join(&name).with_extension("tbl");
        if !try!(metadata(&path)).is_dir()
        {
            let mut file = try!(OpenOptions::new()
                        .read(true)
                        .open(&path));

            let metadata: TableMetadata = try!(decode_from(&mut file, SizeLimit::Infinite));
            info!("loaded table: {}", &name);

            Ok(Table {
                database: db,
                name: try!(Identifier::new(name)),
                meta_data: metadata
            })
        }else {
            error!("could not load table: {} at {}", &name, &path.to_str().unwrap());
            Err(Error::LoadTable)
        }
    }

    // TODO: write table delete static function

    /// Returns a vector of the sizes of the columns
    pub fn get_cols_sizes(&self) -> Vec<u32> {
        let mut column_sizes = Vec::<u32>::new();

        for v in &self.meta_data.columns {
            column_sizes.push(v.size());
        }

        column_sizes
    }

    pub fn get_table_header_offset(&self) -> u32 {
        let bytes: Vec<u8> = encode(&self.meta_data, SizeLimit::Infinite).unwrap();
        bytes.len() as u32 
    }

    pub fn get_cols_offset(&self) -> u32 {
        let mut size = 0;

        for v in &self.meta_data.columns {
            size += v.size();
        }

        size
    }

    /// Get full file path including filename and ext
    pub fn get_path(&self) -> PathBuf {
        self.database.get_path().clone().join(&self.name.get_name()).with_extension("tbl")
    }

    pub fn get_engine_id(&self) -> u8 {
        self.meta_data.engine_id
    }

}

#[cfg(test)]
mod test {
    use super::Table;
    use super::super::database::*;
    use std::fs::metadata;
    use super::super::column::Column;
    use super::super::types::Types;

    #[test]
    fn create_table() {
        let path = "/tmp/test1/";
        let name = "test_db3";
        let mut columns = Vec::<Column>::new();
        columns.push(Column::new("Name", Types::Char(10)));
        columns.push(Column::new("Age", Types::Int));

        let db = Database::create(&name, DatabaseConfig::new(&path)).unwrap();
        let table = Table::create("test_table1", 0, columns, &db).unwrap();

        assert!(!metadata("/tmp/test1/test_db3/test_table1.tbl").unwrap().is_dir());
    }

    #[test]
    fn load_table() {
        let path = "/tmp/test1/";
        let name = "test_db4";

        let mut columns = Vec::<Column>::new();
        columns.push(Column::new("Name", Types::Char(10)));
        columns.push(Column::new("Age", Types::Int));

        let db2 = Database::create(&name, DatabaseConfig::new(&path));
        assert!(db2.is_ok());
        let db = db2.unwrap();
        let table3 = Table::create("test_table2", 34, columns, &db);
        assert!(table2.is_ok());
        let table = table3.unwrap();

       // assert!(!metadata("/tmp/test1/test_db3/test_table2.tbl").unwrap().is_dir());

        let table2 = Table::load("test_table2", &db).unwrap();

        assert_eq!(table.get_engine_id(), table2.get_engine_id());
        assert_eq!(&table.meta_data.columns, &table2.meta_data.columns);
    }

    #[test]
    fn table_offset() {
        let path = "/tmp/test1/";
        let name = "test_db5";


        let mut columns = Vec::<Column>::new();
        columns.push(Column::new("Name", Types::Char(10)));
        columns.push(Column::new("Age", Types::Int));
 
        let db = Database::create(&name, DatabaseConfig::new(&path)).unwrap();
        let table = Table::create("test_table1", 34, columns, &db).unwrap();

        assert_eq!(table.get_table_header_offset(), 42u32);

    }

}
