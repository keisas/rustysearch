use actix_web::{web, HttpResponse, Responder};
use crate::services::{db, relevance, types, types::*};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::time::Instant;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub mode: Option<String>, // "keyword" or "semantic"
}

pub async fn perform_search(
    pool: web::Data<DbPool>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    let query = query.into_inner();
    let start = Instant::now();

    let books = match db::fetch_books_by_title(pool.clone(), &query.query).await {
        Ok(results) => results,
        Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    };

    let scores = match relevance::compute_relevance_scores(&books).await {
        Ok(scores) => scores,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Relevance error: {}", e)),
    };

    let mut results = types::merge_books_with_scores(books, scores);
    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

    HttpResponse::Ok().json(SearchResult {
        results,
        elapsed_time: start.elapsed().as_secs_f64(),
    })
}

pub fn config_search(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search").route(web::get().to(perform_search)));
}
