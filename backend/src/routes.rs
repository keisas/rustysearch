use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::*;
use crate::schema::articles::dsl::*;
use std::time::Instant;
use serde::{Serialize, Deserialize};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Serialize)]
struct SearchResult {
    results: Vec<Article>,
    elapsed_time: f64,
}

async fn perform_search(pool: web::Data<DbPool>, query: web::Query<SearchQuery>) -> impl Responder {
    let query = query.into_inner();
    let start = Instant::now();
    dbg!(query.query.clone());

    let pool = pool.clone();
    let results = web::block(move || {
        // query.search_typeによって処理を変える
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        articles
            .filter(description.ilike(format!("%{}%", query.query)))
            .load::<Article>(&mut conn)
    })
    .await;

    match results {
        Ok(results) => {
            // ここで機械学習を使って結果を加工する
            let elapsed_time = start.elapsed().as_secs_f64();
            HttpResponse::Ok().json(SearchResult { 
                results: results.expect("Couldn't load articles"),
                elapsed_time: elapsed_time,
            })
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config_search(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/search")
            .route(web::get().to(perform_search))
    );
}