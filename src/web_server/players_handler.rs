extern crate iron;
extern crate persistent;
extern crate bodyparser;
extern crate plugin;
extern crate params;

use iron::prelude::*;
use iron::{status, BeforeMiddleware};
use iron::typemap::Key;
use self::params::*;

use game_logic::player::Player;

use std::collections::HashMap;
use web_utils::server_errors::*;
                                       
use web_server::game_sessions_handler::GameSessions;
use std::error::Error;
use std::fmt::{self, Debug};
use std::sync::Arc;

use redis::{Commands, PipelineCommands, transaction, Client};

//Holds file and comments vector
//loads file nad dump it into vector
//keeps adding new comments into vector
//every X comments saves to file
pub struct PlayersHandler{
    registry : HashMap<u32,Arc<Player>>,
}

impl fmt::Display for PlayersHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for (k, v) in &self.registry {
            try!(write!(f, "({}) => {}\n", k, v.get_id()))
        }

        Ok(())
    }
}


impl PlayersHandler{
    pub fn new()->Self{
        PlayersHandler{registry :HashMap::new()}
    }

    pub fn add_player(& mut self, player: Player)
    {
        self.registry.insert(player.get_id(), Arc::new(player));
    }

    pub fn contains(& self, id:&u32)->bool{
        self.registry.contains_key(id)
    }

    pub fn get(& self, id:&u32)->Option<Arc<Player>>{
        self.registry.get(id).cloned()
    }

    pub fn print(& self)
    {
        for (k, v) in &self.registry {
            println!("({}) => {}\n", k, v.get_id())
        }
    }
}

 impl Key for PlayersHandler { type Value = PlayersHandler; }

// impl Handler for PlayersHandler{
//     fn handle(&self, req:&mut Request)-> IronResult<Response>{
//         println!("PlayersHandlerAuthenticator handle");
//         let map = req.get_ref::<Params>().unwrap().clone();
//         let lock : & _ = req.get_ref::<persistent::State<PlayersHandler>>().unwrap();
//         let dynamic_asset = lock.read().unwrap();
//         let player_id : _;
//
//         match map.find(&["userid"]) {
//             Some(&Value::String(ref id)) => player_id = id.parse::<u32>().unwrap_or(0),
//             _ => panic!("Unexpected parameter type!"),
//         }
//
//         match dynamic_asset.contains(&player_id) {
//             true => Err(IronError::new(StringError("Error".to_string()), status::NotFound)),
//             false =>Ok(Response::with((status::Ok, ""))),
//         }
//
//
//     }
// }

pub struct PlayersHandlerAuthenticator;

impl BeforeMiddleware for PlayersHandlerAuthenticator {
    fn before(&self, req: & mut Request) -> IronResult<()> {
        println!("PlayersHandlerAuthenticator before");

        let map = req.get_ref::<Params>().unwrap().clone();
        let lock : & _ = req.get_ref::<persistent::State<PlayersHandler>>().unwrap();
        let dynamic_asset = lock.read().unwrap();
        let player_id : _;

        match map.find(&["userid"]) {
            Some(&Value::String(ref id)) => player_id = id.parse::<u32>().unwrap_or(0),
            _ => {
                return Err(IronError::new(StringError("No player id in param".to_string()), status::NotFound));},
        }

        println!("{:?}", player_id);
        match dynamic_asset.contains(&player_id) {
            true => Ok(()),
            false => Err(IronError::new(StringError("No player id in roster".to_string()), status::NotFound)),
        }
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
        println!("PlayersHandlerAuthenticator BOOM MOTHERFUCKER {:?}",err);
        Err(err)
     }

}


pub fn authenticate_player<'a>(req: &'a mut Request) -> IronResult<Response> {
    println!("authenticate_player");
    
    let map = req.get::<Params>().unwrap();
    let mut player_nick_name = "Not Found".to_string();
    
    // general connection handling
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    
    let player_id : _;

    match map.find(&["user_id"]) {
        Some(&Value::String(ref id)) => player_id = id,
        _ => panic!("Unexpected parameter type!"),
    }
    
    let mut key = format!("{}:{}","user_id", player_id);
    let ret = con.exists(key.clone()).unwrap();
    if ret{
        key = format!("{}:{}:{}","user_id", player_id, "nick_name");
        player_nick_name = con.get(key).unwrap();
    }
    Ok(Response::with((status::Ok, format!("Player {}",player_nick_name))))
}

pub fn add_player<'a>(req: &'a mut Request) -> IronResult<Response> {
    println!("add_player");

    let map = req.get::<Params>().unwrap();
    
    // general connection handling
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    
    let player_nick_name : _;

    match map.find(&["nick_name"]) {
        Some(&Value::String(ref id)) => player_nick_name = id.to_string(),
        _ => panic!("Unexpected parameter type!"),
    }
    
    let players_count : isize = con.incr("player_count",1).unwrap();
    
    let mut key = format!("{}:{}","user_id", players_count);
    let _ : () = con.set(key.clone(),0).unwrap();
    
    key = format!("{}:{}:{}","user_id", players_count,"nick_name");
    let _ : () = con.set(key,player_nick_name).unwrap();
    
    Ok(Response::with((status::Ok, "Added")))
}
