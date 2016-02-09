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

#[cfg(test)]
mod test {
    use super::{Router, Request, Response};

    #[test]
    fn basics() {
        let mut router = Router::new();
        fn handler(req: Request, res: Response) {
            assert!(true);
        }
        router.add(handler);
    }
}
