use actix_web::{web, App, HttpResponse, HttpServer, Responder, web::Data};
use sqlx::postgres::PgPoolOptions;
// use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

async fn search() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is the search endpoint!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/search", web::get().to(search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}