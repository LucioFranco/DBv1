//! the network module for the database
use std::net::{TcpListener, ToSocketAddrs};
use error::Result;

struct Server {
    listener: TcpListener
}

impl Server {
    fn new<A: ToSocketAddrs>(addrs: A) -> Result<Server> {
        Ok(Server {
            listener: try!(TcpListener::bind(addrs))
        })
    }
}
