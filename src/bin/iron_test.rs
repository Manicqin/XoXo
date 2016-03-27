extern crate iron;
//extern crate mount;
//extern crate staticfile;
extern crate router;

use iron::prelude::*;
use iron::status;
//use router::Router;

pub fn handle_comments(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "")))
}

fn main() {

    let mut router = router::Router::new();
    router.any("/submit_comment",handle_comments);

}

/*src/bin/iron_test.rs:17:12: 17:15 error: type mismatch: the type `fn(&mut iron::request::Request<'_, '_>) -> core::result::Result<iron::response::Response, iron::error::IronError> {handle_comments}` implements the trait `for<'r, 'r, 'r> core::ops::Fn<(&'r mut iron::request::Request<'r, 'r>,)>`, but the trait `for<'r, 'r, 'r> core::ops::Fn<(&'r mut iron::request::Request<'r, 'r>,)>` is required (expected struct `iron::request::Request`, found a different struct `iron::request::Request`) [E0281]
src/bin/iron_test.rs:17     router.any("/submit_comment",handle_comments);
*/
