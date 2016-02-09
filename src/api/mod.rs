extern crate hyper;

use std::net::ToSocketAddrs;
use self::hyper::server::{Server, Request, Response};
mod router;

pub struct Api {
    server: Server
}

impl Api {
    pub fn new<To: ToSocketAddrs>(addr: To) -> Self {
        Api { server: Server::http(addr).unwrap() }
    }
}
