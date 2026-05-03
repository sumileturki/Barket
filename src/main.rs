use crate::{
    db::{DbPool, establish_pool}, handlers::{auth::{login, logout, registeruser}, products::{create_product, delete_product, get_allproduct, get_product, update_product}}, middleware::auth::AuthMiddleware, models::user::{NewUser, User}, schema::users::dsl::*
};
use diesel::prelude::*;
use std::env;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, service}};
use dotenvy::dotenv;

mod db;
mod handlers;
mod middleware;
mod models;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_pool(&database_url);

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .service(hello)
                .service(registeruser)
                .service(logout)
                .service(login)
                .service(create_product)
                .service(get_allproduct)
                .service(get_product)
                .service(update_product)
                .service(delete_product)
                // .service(logout)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/me")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("body")
}
