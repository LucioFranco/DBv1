use std::fs::File;
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

impl<'a> Table<'a> {
    pub fn create<'b>(name: &str, db: &'b Database) -> Result<Table<'b>, Error> {
        info!("creating table {}", name);

        let metadata = TableMetadata { EngineID: 0 };

        let mut buf = try!(File::create(db.get_path().clone().join(&name).with_extension("tbl")));

        encode_into(&metadata, &mut buf, SizeLimit::Infinite).unwrap();

        Ok(Table {
            database: &db,
            name: name.to_string(),
            meta_data: metadata.clone()
        })
    }

    pub fn load(name: &str, db: &'a Database) -> Result<Self, Error> {
        // TODO: load tbl file

        info!("loading table: {}", &name);
        Ok(Table {
            database: db,
            name: name.to_string(),
            meta_data: TableMetadata { EngineID: 0 }
        })
    }

    pub fn save(&self) {

    }

    /// Get full file path including filename and ext
    pub fn get_path(&self) -> PathBuf {
        let path = self.database.get_path();
        Path::new(&path).with_file_name(&self.name).with_extension("tbl")
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
        let table = Table::create("test_table1", &db).unwrap();

        assert!(!metadata("/tmp/test1/test_db3/test_table1.tbl").unwrap().is_dir());
    }
}
