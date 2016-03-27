extern crate xoxo_lib;
extern crate diesel;

use diesel::prelude::*;

use xoxo_lib::db::models::Post;
use std::env::args;
use xoxo_lib::db::common::*;

fn main() {
    use xoxo_lib::db::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
