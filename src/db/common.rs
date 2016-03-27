use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::models::{Post, NewPost};

use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use db::schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}
