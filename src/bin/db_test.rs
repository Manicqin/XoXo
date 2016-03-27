#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate xoxo_lib;

use diesel::prelude::*;
use dotenv::dotenv;

use std::env;
use std::io::{stdin, Read};

use xoxo_lib::db::models::{Post, NewPost};
use xoxo_lib::db::common::*;


#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
fn main() {
    let connection = establish_connection();

        println!("What would you like your title to be?");
        let mut title = String::new();
        stdin().read_line(&mut title).unwrap();
        let title = &title[..(title.len() - 1)]; // Drop the newline character
        println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
        let mut body = String::new();
        stdin().read_to_string(&mut body).unwrap();

        let post = create_post(&connection, title, &body);
        println!("\nSaved draft {} with id {}", title, post.id);


}
