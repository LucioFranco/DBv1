use std::fs::File;
use std::rc::Rc;
use std::io::prelude::*;
use std::path::Path;
use std::fs::DirBuilder;

use bincode::rustc_serialize::{encode_into, decode_from, DecodingError};
use bincode::SizeLimit;


pub struct Database {
    path: String,
    metadata_buf: Option<Rc<File>>,
     metadata: DatabaseMetadata
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
 struct DatabaseMetadata {
    name: String,
    tables: u8
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

        let limit = SizeLimit::Infinite;

        encode_into(&Some(self.metadata.clone()), &mut f, limit).unwrap();

        self.metadata_buf = Some(Rc::new(f));
        info!("Added database: {}", self.metadata.name);
    }

    #[warn(dead_code)]
    pub fn load(&mut self) -> Result<(), DecodingError>{
        let mut f = File::open(Path::new(&self.path).join(&self.metadata.name)).unwrap();
        let limit = SizeLimit::Infinite;
        let tmp: DatabaseMetadata = try!(decode_from(&mut f, limit));

        if tmp.name == self.metadata.name.to_string() {
            self.metadata_buf = Some(Rc::new(f));
        }
        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.metadata.name.clone()
    }
}

#[cfg(test)]
mod test {
    use super::Database;
    use std::fs::{File, metadata};
    use std::path::Path;

    #[test]
    fn create_db() {
        let path = "/tmp/test1/";
        let name = "test_db";
        let mut db = Database::new(&path, &name);
        db.create();

        assert!(metadata(&path).is_ok());
        assert!(metadata(&path).unwrap().is_dir());
        assert!(File::open(Path::new(&path).join(&name)).is_ok());
    }

    #[test]
    fn load_db() {
        #![ignore]
        let path = "/tmp/test1/";
        let name = "test_db2";
        let mut db = Database::new(&path, &name);
        db.create();

        let mut db2 = Database::new(&path, &name);
        if let Err(e) = db2.load() {
            println!("{:?}", e);
            assert!(false);
        }

        assert_eq!(db.get_name(), db2.get_name());
    }
}
