use std::fs::{File, OpenOptions, metadata};
use std::path::{Path, PathBuf};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode_into, decode_from};

use super::Error;
use super::database::Database;

pub struct Table<'a> {
    database: &'a Database,
    name: String,
    meta_data: TableMetadata
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
struct TableMetadata {
    EngineID: u8,
}

/// Table representation
impl<'a> Table<'a> {
    /// Create table in database
    pub fn create<'b>(name: &str, engineID: u8, db: &'b Database) -> Result<Table<'b>, Error> {
        info!("creating table: {}", name);

        let metadata = TableMetadata { EngineID: engineID };

        let mut buf = try!(File::create(db.get_path().clone().join(&name).with_extension("tbl")));

        encode_into(&metadata, &mut buf, SizeLimit::Infinite).unwrap();

        Ok(Table {
            database: &db,
            name: name.to_string(),
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
                name: name.to_string(),
                meta_data: metadata
            })
        }else {
            error!("could not load table: {} at {}", &name, &path.to_str().unwrap());
            Err(Error::LoadTable)
        }
    }

    /// Get full file path including filename and ext
    pub fn get_path(&self) -> PathBuf {
        self.database.get_path().clone().join(&self.name).with_extension("tbl")
    }

    pub fn get_engine_id(&self) -> u8 {
        self.meta_data.EngineID
    }

}

#[cfg(test)]
mod test {
    use super::Table;
    use super::super::database::*;
    use std::fs::metadata;

    #[test]
    fn create_table() {
        let path = "/tmp/test1/";
        let name = "test_db3";

        let db = Database::create(&name, DatabaseConfig::new(&path)).unwrap();
        let table = Table::create("test_table1", 0, &db).unwrap();

        assert!(!metadata("/tmp/test1/test_db3/test_table1.tbl").unwrap().is_dir());
    }

    #[test]
    fn load_table() {
        let path = "/tmp/test1/";
        let name = "test_db4";

        let db = Database::create(&name, DatabaseConfig::new(&path)).unwrap();
        let table = Table::create("test_table1", 34, &db).unwrap();

        assert!(!metadata("/tmp/test1/test_db3/test_table1.tbl").unwrap().is_dir());

        let table2 = Table::load("test_table1", &db).unwrap();

        assert_eq!(table.get_engine_id(), table2.get_engine_id());
    }
}
