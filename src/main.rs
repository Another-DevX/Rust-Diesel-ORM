#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use actix_web::{delete, error, get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use self::models::{NewPost, NewPostHandler, Post, PostSimplified};
use self::schema::posts;
use self::schema::posts::dsl::*;

use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyObj {
    title: String,
    body: String,
}

#[get("/")]
async fn index(template_manager: web::Data<tera::Tera>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Error while connecting to DB");

    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {
            println!("{:?}", data);

            let data = data.unwrap();

            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("index.html", &ctx).unwrap())
        }
        Err(err) => HttpResponse::InternalServerError().body("Error al recibir los datos."),
    }
}

#[get("/post/{post_slug}")]
async fn get_post(
    template_manager: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    post_slug: web::Path<String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error while connecting to DB");

    let url_slug = post_slug.into_inner();

    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)).await {
        Ok(data) => {
            println!("{:?}", data);

            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().body("Post not found");
            }

            let data = &data[0];

            let mut ctx = tera::Context::new();
            ctx.insert("post", &data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("read.html", &ctx).unwrap())
        }
        Err(err) => HttpResponse::InternalServerError().body("Error al recibir los datos."),
    }
}

#[get("/create/{post_id}")]
async fn create_post(template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("create.html", &ctx).unwrap())
}

#[post("/create")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("Error while connecting to DB");

    match web::block(move || Post::create_post(&mut conn, &item)).await {
        Ok(data) => {
            println!("{:?}", data);
            return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(err) => HttpResponse::InternalServerError().body("Error al recibir los datos."),
    }
}

#[get("/edit/{post_id}")]
async fn edit_post_template(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
    template_manager: web::Data<tera::Tera>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error while connecting to DB");
    let post_id = post_id.into_inner();

    match web::block(move || posts.filter(id.eq(&post_id)).load::<Post>(&mut conn)).await {
        Ok(data) => {
            println!("{:?}", data);

            let data = data.unwrap();

            let mut ctx = tera::Context::new();
            ctx.insert("post", &data[0]);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("edit.html", &ctx).unwrap())
        }
        Err(err) => HttpResponse::InternalServerError().body("Error al recibir los datos."),
    }
}

#[post("/edit/{post_id}")]
async fn edit_post(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
    mut payload: web::Payload,
) -> impl Responder {
    const MAX_SIZE: usize = 262_144;
    let mut conn = pool.get().expect("Error while connecting to DB");
    let post_id = post_id.into_inner();
    let mut request_body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (request_body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        request_body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<MyObj>(&request_body)?;

    println!(" this is so important {:?}", obj.title);

    match web::block(move || {
        diesel::update(posts.filter(id.eq(post_id)))
            .set((title.eq(&obj.title), body.eq(&obj.body)))
            .get_result::<Post>(&mut conn)
    })
    .await
    {
        Ok(data) => Ok(HttpResponse::Ok().body(format!("{:?}", data))),
        Err(_) => Err(error::ErrorInternalServerError(
            "Error al recibir los datos.",
        )),
    }
}

#[delete("/delete/{post_id}")]
async fn delete_post(pool: web::Data<DbPool>, post_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Error while connecting to DB");
    let post_id = post_id.into_inner();
    match web::block(move || diesel::delete(posts.filter(id.eq(post_id))).execute(&mut conn)).await
    {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(err) => HttpResponse::InternalServerError().body("Error al recibir los datos."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT FOUND");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .service(delete_post)
            .service(create_post)
            .service(edit_post_template)
            .service(edit_post)
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 5173))?
    .run()
    .await
}
