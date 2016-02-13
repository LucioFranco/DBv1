pub use super::hyper::server::{Request, Response};

pub struct Router {
    handlers: Vec<Handler>
}

type Handler = fn(req: Request, res: Response);

impl Router {
    pub fn new() -> Self {
        Router { handlers: Vec::new() }
    }

    pub fn add(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }
}
