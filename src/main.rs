extern crate bincode;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

mod logger;
mod storage;
mod identifier;

use log::LogLevelFilter;
use storage::database::*;
use storage::column::Column;
use storage::table::Table;

fn main() {
    let log = logger::Builder::new(LogLevelFilter::Info);
    log.enable();
    info!("Started Database");

    let db = Database::create("test_db", DatabaseConfig::from("/tmp".to_string())).unwrap();
    Table::create("user", 0, Vec::<Column>::new(), &db).unwrap();

}

pub enum Error {
    NotValidIdentifier
}
