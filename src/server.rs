use api::*;

pub struct Server {
    api: Option<Api>
}

impl Server {
    pub fn new() -> Self {
        Server { api: None }
    }
}
