extern crate xoxo_lib;
extern crate diesel;

use diesel::prelude::*;
use xoxo_lib::db::common::*;
use std::env::args;

fn main() {
    use xoxo_lib::db::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
