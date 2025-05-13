use actix_web::{web, HttpResponse, Responder};
use crate::services::{db, relevance, types, types::*};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::time::Instant;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Deserialize, Debug)]
pub struct SearchQuery {
    pub query: String,
    pub mode: String,
}

pub async fn perform_search(
    pool: web::Data<DbPool>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    println!("Received search query: {:?}", query);

    let query = query.into_inner();
    let start = Instant::now();
    let mut results = match query.mode.as_str() {
        "keyword" => {
            let books = match db::fetch_books_by_title(pool.clone(), &query.query).await {
                Ok(results) => results,
                Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
            };
            println!("Fetched books: {:?}", books);
            types::merge_books_with_dummy_score(books)
        },
        "semantic" => {
            println!("Performing semantic search with query: {}", query.query);
            let relevance_scores = match relevance::compute_relevance_scores(&query.query).await {
                Ok(scores) => scores,
                Err(e) => return HttpResponse::InternalServerError().body(format!("Relevance error: {}", e)),
            };
            let isbn_list: Vec<String> = relevance_scores.iter().map(|score| score.isbn.clone()).collect();
            let books = match db::fetch_books_by_isbns(pool.clone(), isbn_list).await {
                Ok(results) => results,
                Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
            };
            println!("Fetched books: {:?}", books);
            types::merge_books_with_scores(books, relevance_scores)
        },
        _ => {
            return HttpResponse::BadRequest().body("Invalid search mode");
        }
    };
    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
    HttpResponse::Ok().json(SearchResult {
        results,
        elapsed_time: start.elapsed().as_secs_f64(),
    })
}

pub fn config_search(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search").route(web::get().to(perform_search)));
}
