use axum::{
    routing::{get, post},
    Router,
};

pub async fn init() -> Router {
    Router::new()
        .route("/", get(book))
        .route("/upload", post(upload_book))
        .route("/book_id", get(read_book).put(edit_book))
        .route("/book_id/chapter_id", get(read_chapter).put(edit_chapter))
        .route("/download", get(download_book))
        .route("/search", post(search_book))
}

pub async fn book() -> &'static str {
    "book"
}

async fn upload_book() {}

async fn read_book() {}

async fn edit_book() {}

async fn read_chapter() {}

async fn edit_chapter() {}

async fn download_book() {}

async fn search_book() {}
