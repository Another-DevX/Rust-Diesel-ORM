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
    use self::models::{Post, NewPost, PostSimplified};

    // let new_post: NewPost<'_> = NewPost {
    //     title: "Another title",
    //     slug: "second-post",
    //     body: "Hello world again!",
    // };

    // let post: Post = diesel::insert_into(posts::table).values(&new_post).get_result(conn).expect("The insert failed");

    println!("Query off limits");
    let post_response = posts.load::<Post>(conn)
        .expect("Error while executing query");

    for post in post_response {
        println!("{:?}", post)
    }

    
    println!("Query with order by id");
    let post_response = posts.order(id.desc()).load::<Post>(conn).expect("Error while executing query");

    for post in post_response {
        // println!("{}, {}", post.title, post.body)
        println!("{:?}", post);
    }

    println!("Query with especific slug 'where'");
    let post_response = posts.filter(slug.eq("second-post")).load::<Post>(conn).expect("Error while executing query");

    for post in post_response {
        // println!("{}, {}", post.title, post.body)
        println!("{:?}", post);
    }

    println!("Query with especific columns");
    let post_response = posts.select((title, body)).load::<PostSimplified>(conn).expect("Error while executing query");

    for post in post_response {
        // println!("{}, {}", post.title, post.body)
        println!("{:?}", post);
    }

}
