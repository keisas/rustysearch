use crate::models::Book;
use serde::Serialize;

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

pub fn merge_books_with_scores(books: Vec<Book>, scores: Vec<f32>) -> Vec<SearchResultItem> {
    books
        .into_iter()
        .zip(scores.into_iter())
        .map(|(book, score)| SearchResultItem {
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publication_year: book.publication_year,
            publisher: book.publisher,
            image_url: book.image_url,
            relevance_score: score,
        })
        .collect()
}
