extern crate bincode;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

mod api;
mod logger;
mod server;
mod storage;

use api::Api;
use log::LogLevelFilter;
use storage::database::*;
use storage::table::Table;

fn main() {
    let log = logger::Builder::new(LogLevelFilter::Info);
    log.enable();
    info!("Started Database");

    let db = Database::create("test_db", DatabaseConfig::from("/tmp".to_string())).unwrap();
    Table::create("user", 0, &db).unwrap();

}
