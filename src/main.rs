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
use storage::database::Database;

fn main() {
    //let api = Api::new("127.0.0.1:5660");
    let log = logger::Builder::new(LogLevelFilter::Info);
    log.enable();
    info!("Started Database");
    let mut db = Database::new("/tmp/test", "hello");
    db.create();

}
