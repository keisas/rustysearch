use serde::{Serialize, Deserialize};
use diesel::prelude::*;

// #[derive(Queryable, Serialize, Deserialize)]
// pub struct Article {
//     pub id: i32,
//     pub title: String,
//     pub description: String,
// }

#[derive(Queryable, Serialize, Deserialize)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publication_year: String,
    pub publisher: String,
    pub image_url: String,
}
