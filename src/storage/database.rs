use std::path::{Path, PathBuf};
use std::fs::{DirBuilder, metadata};
use super::Error;

#[derive(Clone)]
pub struct Database {
    name: String,
    config: DatabaseConfig
}

impl Database {
    pub fn create(name: &str, config: DatabaseConfig) -> Result<Database, Error> {
        info!("created new database: {}", d.name);
        let d = Database { name: name.to_string(), config: config.clone() };

        DirBuilder::new()
            .recursive(true)
            .create(Path::new(&config.path).join(name)).unwrap();
        
        Ok(d)
    }

    pub fn load(name: &str, config: DatabaseConfig) -> Result<Database, Error>{
        if try!(metadata(Path::new(&config.path).join(name))).is_dir() {
            info!("loaded database: {}", name.to_string());
            Ok(Database { name: name.to_string(), config:  config })
        } else {
            warn!("could not load database: {} at: {}", name, &config.path);
            Err(Error::LoadDatabase)
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> PathBuf {
        Path::new(&self.config.path.clone()).join(&self.name)
    }
}

#[derive(Clone)]
pub struct DatabaseConfig {
    path: String
}

impl DatabaseConfig {
    pub fn new(p: &str) -> Self {
        DatabaseConfig { path: p.to_string() }
    }
}

impl From<String> for DatabaseConfig {
    fn from(p: String) -> Self {
        DatabaseConfig { path: p }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{File, metadata};
    use std::path::Path;

    #[test]
    fn create_db() {
        let path = "/tmp/test1/";
        let name = "test_db";
        let mut db = Database::create(&name, DatabaseConfig::new(&path));

        assert!(metadata(&path).is_ok());
        assert!(metadata(&path).unwrap().is_dir());
    }

    #[test]
    fn load_db() {
        let path = "/tmp/test1/";
        let name = "test_db2";
        let db = Database::create(&name, DatabaseConfig::new(&path)).unwrap();

        let db2 = Database::load(&name, DatabaseConfig::new(&path)).unwrap();

        assert_eq!(db.get_name(), db2.get_name());
    }
}
