extern crate iron;

use iron::prelude::*;
use iron::{Handler, status};

pub struct HelloWorld{
    name:String,
}

impl HelloWorld{
    pub fn new(name:String)->Self{
        HelloWorld{name:name}
    }
}

impl Handler for HelloWorld{
    fn handle(&self, _:&mut Request)-> IronResult<Response>{
        let test= &self.name;
        println!("blablabla");
        //let a = req.extensions.get::<QueryString>().cloned().ok_or("crap".to_string());
        //println!("{:?}", a);
        Ok(Response::with((status::Ok, test.clone())))
    }
}
