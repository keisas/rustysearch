use crate::models::Book;
use crate::schema::books::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::web;
use diesel::result::Error;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn fetch_books_by_title(
    pool: web::Data<DbPool>,
    query: &str,
) -> Result<Vec<Book>, Error> {
    let query = query.to_string();
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| Error::NotFound)?;
        books
            .filter(title.ilike(format!("%{}%", query)))
            .load::<Book>(&mut conn)
    })
    .await
    .map_err(|_| Error::NotFound)?;

    Ok(result?)
}

pub async fn fetch_books_by_isbns(
    pool: web::Data<DbPool>,
    isbns_list: Vec<String>,
) -> Result<Vec<Book>, Error> {
    let isbns_clone = isbns_list.to_owned();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| Error::NotFound)?;
        books
            .filter(isbn.eq_any(isbns_clone))
            .load::<Book>(&mut conn)
    })
    .await
    .map_err(|_| Error::NotFound)?;

    Ok(result?)
}
