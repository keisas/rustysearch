use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
// use diesel::dsl::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::*;
use crate::schema::books::dsl::*;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use std::process::Command;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Serialize)]
struct SearchResult {
    results: Vec<SearchResultItem>,
    elapsed_time: f64,
}

#[derive(Serialize)]
struct SearchResultItem {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publication_year: String,
    pub publisher: String,
    pub image_url: String,
    pub relevance_score: f32,
}

async fn perform_search(pool: web::Data<DbPool>, query: web::Query<SearchQuery>) -> impl Responder {
    let query = query.into_inner();
    let start = Instant::now();
    dbg!(query.query.clone());

    let pool = pool.clone();
    let articles_results = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        books
            .filter(title.ilike(format!("%{}%", query.query)))
            .load::<Book>(&mut conn)
    })
    .await;

    let book_vec: Vec<Book> = match articles_results {
        Ok(results) => results.expect("Couldn't load articles"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Internal server error: {}", e)),
    };

    let output = Command::new("python3")
        .arg("scripts/calculate_relevance.py")
        .arg(serde_json::to_string(&book_vec).unwrap())
        .output()
        .expect("Failed to execute Python script");

    let relevance_scores: Vec<f32> = serde_json::from_slice(&output.stdout).unwrap();

    let mut results: Vec<SearchResultItem> = book_vec.into_iter().zip(relevance_scores.into_iter()).map(|(book, score)| {
        SearchResultItem {
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publication_year: book.publication_year,
            publisher: book.publisher,
            image_url: book.image_url,
            relevance_score: score,
        }
    }).collect();

    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

    let elapsed_time = start.elapsed().as_secs_f64();
    HttpResponse::Ok().json(SearchResult { 
        results: results,
        elapsed_time: elapsed_time,
    })
}

pub fn config_search(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/search")
            .route(web::get().to(perform_search))
    );
}