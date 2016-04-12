extern crate iron;
extern crate router;
extern crate params;
extern crate persistent;

use iron::BeforeMiddleware;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;

use params::*;

pub struct RosterHandler;
impl Key for RosterHandler { type Value = RosterHandler; }

pub struct SessionsHandler;
impl Key for SessionsHandler { type Value = SessionsHandler; }

pub struct UsersHandler;

impl UsersHandler{
    fn contains(&self, user_id : u32) -> bool
    {
        true
    }
}

impl BeforeMiddleware for UsersHandler {
    fn before(&self, req: & mut Request) -> IronResult<()> {
        println!("UsersHandler before");

        let map = req.get::<Params>().unwrap();
        let roster_lock = req.get::<persistent::State<RosterHandler>>().unwrap();
        let roster = roster_lock.read().unwrap();
        
        let sessions_lock = req.get::<persistent::State<SessionsHandler>>().unwrap();
        let sessions = sessions_lock.read().unwrap();
        
        let user_id : u32 = 0;
        
        // match map.find(&["userid"]) {
        //     Some(&Value::String(ref id)) => user_id = id.parse::<u32>().unwrap_or(0),
        //     _ => {panic!("Just for the example");},
        // }

        // println!("{:?}", user_id);
        // match roster_lock.contains(user_id) {
        //     true => Ok(()),
        //     false => {panic!("Just for the example");},
        // }
        Ok(())
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
        println!("PlayersHandlerAuthenticator BOOM MOTHERFUCKER");
        Err(err)
        //Ok(())
     }

}

fn main() {

    let mut router = router::Router::new();
    //router.any("/submit_comment",handle_comments);

}
