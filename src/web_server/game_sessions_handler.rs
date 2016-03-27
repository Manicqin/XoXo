extern crate iron;
//extern crate persistent;
//extern crate bodyparser;
extern crate plugin;
//extern crate params;

use iron::prelude::*;
use iron::{Handler, status, BeforeMiddleware};
use iron::typemap::Key;
use std::sync::Arc;
use game_logic::game_session::GameSession;
//game_session::GameSession;

pub struct GamSessions{
    registry : Vec<Arc<GameSession>>,
}
pub struct GameSessionFinder;

impl Key for GameSessionFinder { type Value = GameSessionFinder; }
impl Key for GamSessions { type Value = GamSessions; }

impl GamSessions{
    pub fn new()->Self{
        GamSessions{registry:vec![] }
    }

    pub fn push(&mut self,session: GameSession){
        self.registry.push(Arc::new(session));
    }
}
impl Handler for GameSessionFinder{
    fn handle(&self, _:&mut Request)-> IronResult<Response>{
        println!("GameSessionFinder handle");

        Ok(Response::with((status::Ok, "")))


    }
}


impl BeforeMiddleware for GameSessionFinder {
    fn before(&self, _: & mut Request) -> IronResult<()> {
        println!("GameSessionFinder Before");
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        println!("GameSessionFinder BOOM MOTHERFUCKER");
        Err(err)
    }
}
