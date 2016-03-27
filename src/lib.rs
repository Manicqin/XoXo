#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate iron;
extern crate persistent;
//extern crate bodyparser;
extern crate rustc_serialize;

extern crate time;

use diesel::prelude::*;
use diesel::pg::PgConnection;


pub mod utils{
    pub mod config;
}

pub mod game_logic{
    pub mod board;
    pub mod game_session;
    pub mod player;
    pub mod simple_game_board;
}

pub mod db{
    pub mod schema;
    pub mod models;
    pub mod common;
}

pub mod web_utils{
    pub mod iron_router;
    pub mod server_errors;
    pub mod web_logger;
}

pub mod web_server{
    pub mod comments_wrapper;
    pub mod game_sessions_handler;
    pub mod hello_world;
    pub mod players_handler;
}
