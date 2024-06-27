use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::articles;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub description: &'a str,
}