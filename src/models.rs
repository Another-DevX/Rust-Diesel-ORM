use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostSimplified {
    pub title: String,
    pub body: String,
}
#[derive(Queryable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}


use diesel::prelude::*;
use super::schema::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn sluglify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post<'a> (
        conn: &mut PgConnection, 
        post: &NewPostHandler
    ) -> Result<Post, diesel::result::Error> {

        let slug = Post::sluglify(&post.title.clone());

        println!("{}, {}, {}" , &post.title, &slug, &post.body);

        let new_post = NewPost{
            title: &post.title,
            slug: &slug,
            body: &post.body
        };

        diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
    }
}
