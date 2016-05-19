extern crate iron;
extern crate persistent;
extern crate bodyparser;
extern crate plugin;
extern crate params;
extern crate mime;

use iron::prelude::*;
use iron::{Handler, status, BeforeMiddleware, modifier, headers};
use iron::typemap::Key;
use iron::modifiers::Header;
use self::params::*;
use std::sync::Arc;

use web_utils::server_errors::*;
use game_logic::game_session::GameSession;

use redis::{Commands, PipelineCommands, transaction, Client};

#[derive(Debug)]
pub struct GameSessions{
    registry : Vec<Arc<GameSession>>,
}
pub struct GameSessionFinder;

impl Key for GameSessionFinder { type Value = GameSessionFinder; }
impl Key for GameSessions { type Value = GameSessions; }

impl GameSessions{
    pub fn new()->Self{
        GameSessions{registry:vec![] }
    }

    pub fn push(&mut self,session: Arc<GameSession>){
        self.registry.push(session);
    }

    pub fn find_player_games(&self,player_id: u32)->Vec<Arc<GameSession>>{
        self.registry.iter()
            .filter(|x| (x).contains_player(&player_id)).cloned()
            .collect::<Vec<_>>()
    }
    
    pub fn find_game(&self,game_id: u32)->Vec<Arc<GameSession>>{
        self.registry.iter()
            .filter(|x| (x).get_id() == game_id).cloned()
            .inspect(|x| println!("{:?}",x))
            .collect::<Vec<_>>()
    }
}

impl Handler for GameSessionFinder{
    fn handle(&self, _:&mut Request)-> IronResult<Response>{
        println!("GameSessionFinder handle");

        Ok(Response::with((status::Ok, "")))


    }
}

impl<'a,'b> modifier::Modifier<Request<'a, 'b>> for GameSessions
{
    fn modify(self, req: &mut Request) {
        //res.headers.set(self.0);
        req.extensions.insert::<GameSessions>(self);
    }
}
    
impl BeforeMiddleware for GameSessionFinder {
        
    fn before(&self, req: & mut Request) -> IronResult<()> {
        println!("GameSessionFinder Before");
       
        let map = req.get::<Params>().unwrap();
        
        let lock = req.get::<persistent::State<GameSessions>>().unwrap();
        let dynamic_asset = lock.read().unwrap();
        let game_id : _;

        match map.find(&["gameid"]) {
            Some(&Value::String(ref id)) => game_id = id.parse::<u32>().unwrap_or(0),
            _ => {
                return Err(IronError::new(StringError("No gameid was given".to_string()), status::NotFound));},
        }
        
        let mut player_sessions = GameSessions::new();
        
        for game in dynamic_asset.find_player_games(game_id).iter(){
            println!("{:?}",game);
            player_sessions.push(game.clone());
        } 
        req.set_mut(player_sessions);
        
        
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        println!("GameSessionFinder BOOM MOTHERFUCKER {:?}",err);
        Err(err)
    }
}

pub fn print_last_game<'a>(req: &'a mut Request) -> IronResult<Response> {
    println!("print_game");
    
    let lock = req.get::<persistent::State<GameSessions>>().unwrap();
    let dynamic_asset = lock.read().unwrap();

    let mut game_id : u32 = 0;
    
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    
    let mut game_id_str : String = con.get("games_count").unwrap();
    
    game_id = game_id_str.parse::<u32>().unwrap();
    
    let mut out : String = "Not Found".to_string();
    
    let games = dynamic_asset.find_game(game_id);
    if games.len() > 0{
         out = games[0].make_draw();
    }

    let mut res = Response::with((status::Ok, format!("{}",out)));
    let mime_ = mime!(Text/Html);
    
    res.set_mut(Header(headers::ContentType(mime_)));
    Ok(res)
}

pub fn print_game<'a>(req: &'a mut Request) -> IronResult<Response> {
    println!("print_game");
    
    let map = req.get::<Params>().unwrap();
    let lock = req.get::<persistent::State<GameSessions>>().unwrap();
    let dynamic_asset = lock.read().unwrap();

    let game_id : _;

    match map.find(&["game_id"]) {
        Some(&Value::String(ref id)) => game_id = id.parse::<u32>().unwrap(),
        _ => panic!("Unexpected parameter type!"),
    }
    
    let mut out : String = "Not Found".to_string();
    
    let games = dynamic_asset.find_game(game_id);
    
    if games.len() > 0{
         out = games[0].make_draw();
    }

    let mut res = Response::with((status::Ok, format!("{}",out)));
    let mime_ = mime!(Text/Html);
    
    res.set_mut(Header(headers::ContentType(mime_)));
    Ok(res)
}

pub fn add_game<'a>(req: &'a mut Request) -> IronResult<Response> {
    println!("add_game");

    let map = req.get::<Params>().unwrap();
    let lock = req.get::<persistent::State<GameSessions>>().unwrap();
        
    // general connection handling
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    
    let player1_id : _;
    let player2_id : _;
    let mut game_id : usize = 0;
    
    match map.find(&["user1_id"]) {
        Some(&Value::String(ref id)) => player1_id = id.parse::<u32>().unwrap_or(0),
        _ => panic!("Unexpected user1_id parameter type!"),
    }

    match map.find(&["user2_id"]) {
        Some(&Value::String(ref id)) => player2_id = id.parse::<u32>().unwrap_or(0),
        _ => panic!("Unexpected user2_id parameter type!"),
    }
    
    let player1_exists = con.exists(format!("{}:{}:{}","user_id", player1_id,"nick_name")).unwrap();
    let player2_exists = con.exists(format!("{}:{}:{}","user_id", player2_id,"nick_name")).unwrap();
    
    if  player1_exists && player2_exists{
        
        game_id = con.incr("games_count",1).unwrap();

        let mut key = format!("{}:{}","game_id", game_id);
        let _ : () = con.set(key.clone(),0).unwrap();
        
        key = format!("{}:{}",key,"player1");
        let _ : () = con.set(key.clone(), player1_id).unwrap();
        
        key = format!("{}:{}","game_id", game_id);
        key = format!("{}:{}",key,"player2");
        let _ : () = con.set(key,player2_id).unwrap();
        
        let mut dynamic_asset = lock.write().unwrap();
        
        println!("pushing {:?}",game_id);
        let mut session = GameSession::new(game_id as u32,3);
        dynamic_asset.push(Arc::new(session));
        println!("{:?}",*dynamic_asset);

        Ok(Response::with((status::Ok, format!("Game {} Added", game_id))))
    }
    else{
        Ok(Response::with((status::Ok, "Failed")))
    }
    
    
}