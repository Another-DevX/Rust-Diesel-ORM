#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::posts;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT FOUND");

    let conn = &mut PgConnection::establish(&db_url).expect("Error while connecting to DB");

    use self::schema::posts::dsl::*;
    use self::schema::posts;
    use self::models::{Post, NewPost};

    let new_post: NewPost<'_> = NewPost {
        title: "Another title",
        slug: "second-post",
        body: "Hello world again!",
    };

    // let post: Post = diesel::insert_into(posts::table).values(&new_post).get_result(conn).expect("The insert failed");

    // Select * from posts limit 1
    let post_response = posts.limit(1).load::<Post>(conn)
        .expect("Error while executing query");

    for post in post_response {
        println!("{}", post.title)
    }
}
