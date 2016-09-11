//! the network module for the database
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

use error::Result;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new<A: ToSocketAddrs>(addrs: A) -> Result<Server> {
        Ok(Server {
            listener: try!(TcpListener::bind(addrs))
        })
    }

    pub fn handle<F>(&mut self, f: F) -> Result<()>
        where F: FnOnce(TcpStream) -> Result<()> {
        for stream in self.listener.incoming() {
            let stream = try!(stream);

            thread::spawn(move || {
               f(stream);
            });
        }

        Ok(())
    }
}
