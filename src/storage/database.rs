use std::path::Path;
use std::fs::{DirBuilder, metadata};
use super::Error;

pub struct Database {
    name: String
}

impl Database {
    pub fn create(name: &str, path: &str) -> Result<Database, Error> {
        let d = Database { name: name.to_string() };
        DirBuilder::new()
            .recursive(true)
            .create(Path::new(path).join(name)).unwrap();

        info!("created new database: {}", d.name);
        Ok(d)
    }

    pub fn load(name: &str, path: &str) -> Result<Database, Error>{
        if try!(metadata(Path::new(path).join(name))).is_dir() {
            info!("loaded database: {}", name.to_string());
            Ok(Database { name: name.to_string() })
        } else {
            warn!("could not load database: {} at: {}", name, path);
            Err(Error::LoadDatabase)
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
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
        let mut db = Database::create(&name, &path);

        assert!(metadata(&path).is_ok());
        assert!(metadata(&path).unwrap().is_dir());
    }

    #[test]
    fn load_db() {
        let path = "/tmp/test1/";
        let name = "test_db2";
        let db = Database::create(&name, &path).unwrap();

        let db2 = Database::load(&name, &path).unwrap();

        assert_eq!(db.get_name(), db2.get_name());
    }
}
