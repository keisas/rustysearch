diesel::table! {
    books (isbn) {
        isbn -> Varchar,
        title -> Varchar,
        author -> Varchar,
        publication_year -> Varchar,
        publisher -> Varchar,
        image_url -> Varchar,
    }
}