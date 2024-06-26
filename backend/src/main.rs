use actix_web::{web, App, HttpResponse, HttpServer, Responder, web::Data};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use std::process::Command;
use std::time::Instant;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
    search_type: String,
    use_ml: bool,
}


#[derive(Serialize, Deserialize)]
struct SearchResultItem {
    id: i32,
    title: String,
    description: String,
    relevance_score: f32,
}


#[derive(Serialize)]
struct SearchResult {
    results: Vec<SearchResultItem>,
    elapsed_time: f64,
}

async fn search(pool: web::Data<PgPool>,
                query: web::Query<SearchQuery>
) -> impl Responder {
    let query = query.into_inner();
    let start = Instant::now();
    dbg!(query.query.clone());
    
    let mut results = match query.search_type.as_str() {
        "fulltext" => {
            dbg!("fulltext search");
            let results = sqlx::query!(
                "SELECT id, title, description
                 FROM articles
                 WHERE tsv @@ plainto_tsquery('english', $1)",
                query.query
            )
            .fetch_all(pool.as_ref())
            .await
            .expect("Failed to execute query");
    
            results.into_iter().map(|r| SearchResultItem {
                id: r.id,
                title: r.title,
                description: r.description,
                relevance_score: 0.0,
            }).collect::<Vec<SearchResultItem>>()
        },
        "index" => {
            dbg!("index search");
            let results = sqlx::query!(
                "SELECT id, title, description
                 FROM articles
                 WHERE title ILIKE $1",
                format!("%{}%", query.query)
            )
            .fetch_all(pool.as_ref())
            .await
            .expect("Failed to execute query");
    
            results.into_iter().map(|r| SearchResultItem {
                id: r.id,
                title: r.title,
                description: r.description,
                relevance_score: 0.0,
            }).collect::<Vec<SearchResultItem>>()
        },
        _ => {
            dbg!("sequential search");
            let results = sqlx::query!(
                "SELECT id, title, description
                 FROM articles
                 WHERE description ILIKE $1",
                format!("%{}%", query.query)
            )
            .fetch_all(pool.as_ref())
            .await
            .expect("Failed to execute query");
    
            results.into_iter().map(|r| SearchResultItem {
                id: r.id,
                title: r.title,
                description: r.description,
                relevance_score: 0.0,
            }).collect::<Vec<SearchResultItem>>()
        }
    };

    if query.use_ml {
        let output = Command::new("python3")
            .arg("scripts/search_model.py")
            .arg(serde_json::to_string(&results).unwrap())
            .output()
            .expect("Failed to execute command");
        
        results = serde_json::from_slice(&output.stdout)
            .unwrap_or_else(|_| vec![]);
    }

    let elapsed_time = start.elapsed().as_secs_f64();

    HttpResponse::Ok().json(SearchResult { results, elapsed_time })
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}