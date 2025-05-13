use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;
mod routes;
mod services;

use routes::search::config_search;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = web::Data::new(establish_connection_pool());

    // 起動したことを知らせるログ
    println!("Starting server at: http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(pool.clone())
            .configure(config_search)
    })
    // .bind("0.0.0.0:8080")?
    .bind("backend:8080")?
    .run()
    .await
}