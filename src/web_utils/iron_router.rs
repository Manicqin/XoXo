use iron::prelude::*;
use iron::{Handler, status};
use std::collections::HashMap;

pub struct Router2 {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<Handler>>
}

impl Router2 {
    pub fn new() -> Self {
        Router2 { routes: HashMap::new() }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router2 {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}
