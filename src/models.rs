#[derive(Queryable)]
#[derive(Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostSimplified {
    pub title: String,
    pub body: String,
}
#[derive(Queryable)]
#[derive(Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}
