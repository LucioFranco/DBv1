use std::fs::File;
use std::rc::Rc;
use std::io::prelude::*;
use std::path::Path;
use std::fs::DirBuilder;

use bincode::rustc_serialize::{encode_into};
use bincode::SizeLimit;


pub struct Database {
    path: String,
    metadata_buf: Option<Rc<File>>,
    metadata: DatabaseMetadata
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
struct DatabaseMetadata {
    name: String,
    tables: u16
}

impl Database {
    pub fn new(p: &str, name: &str) -> Self {
        Database {
            path: p.to_string(),
            metadata_buf: None,
            metadata: DatabaseMetadata {
                name: name.to_string(),
                tables: 0
            }
        }
    }

    pub fn create(&mut self) {
        DirBuilder::new()
            .recursive(true)
            .create(&self.path).unwrap();

        let mut f: File = File::create(Path::new(&self.path).join(&self.metadata.name)).unwrap();

        let limit = SizeLimit::Bounded(20);

        encode_into(&Some(self.metadata.clone()), &mut f, limit).unwrap();

        self.metadata_buf = Some(Rc::new(f));
        info!("Added database: {}", self.metadata.name);
    }

    #[allow(dead_code)]
    pub fn load(&mut self) {
        let mut f = File::open("/tmp/database_test".to_string() + &self.metadata.name).unwrap();
        let mut name = String::new();
        f.read_to_string(&mut name).unwrap();

        if name == self.metadata.name.to_string() {
            self.metadata_buf = Some(Rc::new(f));
        }
    }
}

#[cfg(test)]
mod test {
    use super::Database;
    use std::fs::{File, metadata};

    #[test]
    fn create_db() {
        let path = "/tmp/test1/";
        let mut db = Database::new(&path, "test_db");
        db.create();

        assert!(metadata(&path).is_ok());
        assert!(metadata(&path).unwrap().is_dir());
        assert!(File::open("/tmp/test1/test_db").is_ok());
    }
}
