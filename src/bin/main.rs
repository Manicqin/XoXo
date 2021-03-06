extern crate xoxo_lib;

extern crate rand;
extern crate xml;
extern crate time;

extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate router;
extern crate persistent;

use iron::prelude::*;
use iron::AroundMiddleware;
use mount::Mount;
use staticfile::Static;
use router::Router;
use persistent::State;

use xoxo_lib::game_logic::*;
use xoxo_lib::utils::config::{Config};
use xoxo_lib::web_utils::*;
use xoxo_lib::web_server::*;

use std::sync::Arc;

#[cfg(not(test))]
fn main() {

    let mut args = std::env::args();
    let running_path = args.next();
    let path = args.next();

    println!("running from: {:?}", running_path);
    let path = if let Some(ref path) = path {
        path
    } else {
        println!("Usage: <file>");
        return;
    };

    let cnfg = Config::new().init(path);

    let log_level       = cnfg.get("log_level").unwrap().parse::<usize>().unwrap_or(0);
    let port            = cnfg.get("port").unwrap();
    let ip              = cnfg.get("ip").unwrap();
    let template_path   = cnfg.get("template_path").unwrap().to_owned();
    let scripts_path    = cnfg.get("scripts_path").unwrap().to_owned();
    let static_path     = cnfg.get("static_path").unwrap().to_owned();
    let logger_mode     = web_logger::Logger::parse_logger_mode(log_level as u8);
    
    let mut main_router = Router::new();
    main_router.get("game/hello".to_string(), hello_world::HelloWorld::new("shit got real".to_string()));
    main_router.get("game/user", players_handler::authenticate_player);

    main_router.get("game/add_player", players_handler::add_player);
    main_router.get("game/add_game", game_sessions_handler::add_game);
    main_router.get("game/print_game", game_sessions_handler::print_game);
    main_router.get("game/print_last_game", game_sessions_handler::print_last_game);
    
    let mut main_chain = Chain::new(main_router);
    let mut sessions = game_sessions_handler::GameSessions::new();
    main_chain.link_before(persistent::State::<game_sessions_handler::GameSessions>::one(sessions));
    
    let mut mount = Mount::new();
    mount.mount("/scripts/",Static::new(scripts_path));
    mount.mount("/static-pages/",Static::new(static_path.clone()));
    mount.mount("/favicon.ico",Static::new(static_path + "favicon.ico"));
    mount.mount("/",main_chain);

    //curl -i "localhost:6767/" -H "application/json" -d '{"author":"jason","text":"2kjhkjhjh kjh khkj kj hkjh kh kmhghg jjhghg fhg 98654345678987654"}'
    let server = Iron::new(web_logger::Logger::new(logger_mode).around(Box::new(mount)));
    server.http(format!("{}:{}",ip,port).trim()).unwrap();
}
