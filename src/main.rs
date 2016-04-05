extern crate bincode;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

pub mod logger;
pub mod storage;
pub mod identifier;
pub mod parse;
pub mod error;

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
