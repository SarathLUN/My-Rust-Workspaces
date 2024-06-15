use actix_web::{web, App, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;

pub mod handlers;
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Fail to create database pool.");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
            .configure(handlers::init)
    })
        .bind("127.0.0.1:8080")?
    .run()
        .await

}
