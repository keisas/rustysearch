use crate::models::Book;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct SearchResultItem {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publication_year: String,
    pub publisher: String,
    pub image_url: String,
    pub relevance_score: f32,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub results: Vec<SearchResultItem>,
    pub elapsed_time: f64,
}

#[derive(Deserialize)]
pub struct RelevanceScore {
    pub isbn: String,
    pub relevance_score: f32,
}

pub fn merge_books_with_scores(
    books: Vec<Book>,
    scores: Vec<RelevanceScore>,
) -> Vec<SearchResultItem> {

    let score_map: HashMap<String, f32> =
        scores.into_iter().map(|s| (s.isbn, s.relevance_score)).collect();

    books
        .into_iter()
        .map(|book| {
            let score = score_map.get(&book.isbn).copied().unwrap_or(0.0);
            SearchResultItem {
                isbn: book.isbn,
                title: book.title,
                author: book.author,
                publication_year: book.publication_year,
                publisher: book.publisher,
                image_url: book.image_url,
                relevance_score: score,
            }
        })
        .collect()
}

pub fn merge_books_with_dummy_score(books: Vec<Book>) -> Vec<SearchResultItem> {
    books
        .into_iter()
        .map(|book| SearchResultItem {
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publication_year: book.publication_year,
            publisher: book.publisher,
            image_url: book.image_url,
            relevance_score: 0.0, 
        })
        .collect()
}
