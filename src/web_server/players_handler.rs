extern crate iron;
//extern crate persistent;
//extern crate bodyparser;
extern crate plugin;
//extern crate params;

use iron::prelude::*;
use iron::{status, BeforeMiddleware};
use iron::typemap::Key;
//use params::{Params, Value};

use game_logic::player::Player;
use std::collections::HashMap;

use std::error::Error;
use std::fmt::{self, Debug};
use std::sync::Arc;

#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

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

    //     let map = req.get_ref::<Params>().unwrap().clone();
    //     let lock : & _ = req.get_ref::<persistent::State<PlayersHandler>>().unwrap();
    //     let dynamic_asset = lock.read().unwrap();
    //     let player_id : _;
    //
    //     match map.find(&["userid"]) {
    //         Some(&Value::String(ref id)) => player_id = id.parse::<u32>().unwrap_or(0),
    //         _ => panic!("Unexpected parameter type!"),
    //     }
    //
    //     match dynamic_asset.contains(&player_id) {
    //         true => Ok(()),
    //         false => Err(IronError::new(StringError("No player id".to_string()), status::NotFound)),
    //     }
    //
    //
    //
    // }
    //
    // fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
    //     println!("PlayersHandlerAuthenticator BOOM MOTHERFUCKER");
    //     Err(err)
        Ok(())
     }

}


pub fn auhentcate_player<'a>(req: &'a mut Request) -> IronResult<Response> {
    println!("auhentcate_player");
    Ok(Response::with((status::Ok, format!("{:?}",req))))
}
