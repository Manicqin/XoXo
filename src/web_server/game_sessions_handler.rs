extern crate iron;
extern crate persistent;
extern crate bodyparser;
extern crate plugin;
extern crate params;

use iron::prelude::*;
use iron::{Handler, status, BeforeMiddleware};
use iron::typemap::Key;
use self::params::*;
use std::sync::Arc;

use web_utils::server_errors::*;
use game_logic::game_session::GameSession;
//game_session::GameSession;

#[derive(Debug)]
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

    pub fn find_player_games(&self,player_id: u32)->Vec<&Arc<GameSession>>{
        self.registry.iter()
            .filter(|x| (**x).contains_player(&player_id))
            .collect::<Vec<_>>()
    }
}
impl Handler for GameSessionFinder{
    fn handle(&self, _:&mut Request)-> IronResult<Response>{
        println!("GameSessionFinder handle");

        Ok(Response::with((status::Ok, "")))


    }
}

// fn bla(req : & Request) ->  Params::Map {
//     Params::Map::new()
// }
    
impl BeforeMiddleware for GameSessionFinder {
        
    fn before(&self, req: & mut Request) -> IronResult<()> {
        println!("GameSessionFinder Before");
       
        let map = req.get::<Params>().unwrap();
        
        let lock = req.get::<persistent::State<GamSessions>>().unwrap();
        let dynamic_asset = lock.read().unwrap();
        let game_id : _;

        match map.find(&["gameid"]) {
            Some(&Value::String(ref id)) => game_id = id.parse::<u32>().unwrap_or(0),
            _ => {
                return Err(IronError::new(StringError("No gameid was given".to_string()), status::NotFound));},
        }
        
        println!(",fhdskjfhjdfsh {:?}", *dynamic_asset);
        for game in dynamic_asset.find_player_games(game_id).iter(){
            println!("{:?}",game);
        } 
        
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        println!("GameSessionFinder BOOM MOTHERFUCKER {:?}",err);
        Err(err)
    }
}
