use actix_web::{web, App, HttpResponse, HttpServer, Responder, web::Data};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
// use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

// #[derive(Serialize)]
// struct SearchResult {
//     results: Vec<String>
// }

async fn search(pool: web::Data<PgPool>,
                query: web::Query<SearchQuery>
) -> impl Responder {
    let query = query.into_inner().query;
    let results = sqlx::query!(
        "SELECT results FROM searches WHERE query = $1",
        query
    )
    .fetch_all(pool.as_ref())
    .await
    .map(|rows| {
        rows.into_iter()
            .flat_map(|row| {
                serde_json::from_value::<Vec<String>>(row.results.unwrap_or(JsonValue::Null)).unwrap_or_else(|_| vec![])
            })
            .collect()
    })
    .unwrap_or_else(|_| vec![]);
    HttpResponse::Ok().json(results)
}

#[derive(Deserialize)]
struct NewSearch {
    query: String,
    results: Vec<String>,
}

async fn add_search(pool: web::Data<PgPool>,
                    new_search: web::Json<NewSearch>
) -> impl Responder {
    let new_search = new_search.into_inner();
    let query = new_search.query;
    let results = serde_json::to_value(new_search.results).unwrap();
    let _ = sqlx::query!(
        "INSERT INTO searches (query, results) VALUES ($1, $2)",
        query,
        results
    )
    .execute(pool.as_ref())
    .await;

    HttpResponse::Ok().body("Search added")
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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .route("/search", web::get().to(search))
            .route("/search", web::post().to(add_search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}