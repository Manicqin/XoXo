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

     let mut roster = players_handler::PlayersHandler::new();
     roster.add_player(player::Player::new(10));
     roster.add_player(player::Player::new(11));
     roster.add_player(player::Player::new(12));
     roster.add_player(player::Player::new(13));
     roster.add_player(player::Player::new(14));

     let mut session = game_session::GameSession::new(3);
     session.attach_player(roster.get(&10).unwrap());
     session.attach_player(roster.get(&11).unwrap());

     let mut sessions = game_sessions_handler::GamSessions::new();
     sessions.push(session);

      let mut main_router = Router::new();
      main_router.get("game/hello".to_string(), hello_world::HelloWorld::new("shit got real".to_string()));
      main_router.get("game/user", players_handler::auhentcate_player);

      let mut main_chain = Chain::new(main_router);

      main_chain.link_before(persistent::State::<players_handler::PlayersHandler>::one(roster));
      main_chain.link_before(persistent::State::<game_sessions_handler::GamSessions>::one(sessions));
      main_chain.link_before(players_handler::PlayersHandlerAuthenticator);
      main_chain.link_before(game_sessions_handler::GameSessionFinder);

      let mut mount = Mount::new();
      mount.mount("/scripts/",Static::new(scripts_path));
      mount.mount("/static-pages/",Static::new(static_path.clone()));
      mount.mount("/favicon.ico",Static::new(static_path + "favicon.ico"));
      mount.mount("/",main_chain);

     //curl -i "localhost:6767/" -H "application/json" -d '{"author":"jason","text":"2kjhkjhjh kjh khkj kj hkjh kh kmhghg jjhghg fhg 98654345678987654"}'
     let server = Iron::new(web_logger::Logger::new(logger_mode).around(Box::new(mount)));
     server.http(format!("{}:{}",ip,port).trim()).unwrap();
}
