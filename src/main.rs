extern crate rust_db;
#[macro_use] extern crate log;

use log::LogLevelFilter;
use rust_db::storage::database::*;
use rust_db::storage::column::Column;
use rust_db::storage::table::Table;
use rust_db::logger;

fn main() {
    let log = logger::Builder::new(LogLevelFilter::Info);
    log.enable();
    info!("Started Database");

    let db = Database::create("test_db", DatabaseConfig::from("/tmp".to_owned())).unwrap();
    Table::create("user", 0, Vec::<Column>::new(), &db).unwrap();

}
