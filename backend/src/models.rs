use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publication_year: String,
    pub publisher: String,
    pub image_url: String,
}
