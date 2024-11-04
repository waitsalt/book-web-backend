use crate::{
    component::model::book::UploadBook,
    util::error::{AppError, BookError},
};
use axum::{
    extract::{Multipart, Path},
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

/*
curl http://127.0.0.1:8000/book
*/
pub async fn book() -> &'static str {
    "book"
}

async fn upload_book(mut multipart: Multipart) -> Result<&'static str, AppError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let upload_book: UploadBook = serde_json::from_slice(&data)
            .map_err(|_| AppError::BookError(BookError::UploadFileFormatError))?;
        upload_book.save_to_file().await?;
        return Ok("success");
    }
    Err(AppError::BookError(BookError::NoUploadFile))
}

async fn read_book(Path(book_id): Path<String>) {}

async fn edit_book(Path(book_id): Path<String>) {}

async fn read_chapter(Path((book_id, chapter_id)): Path<(String, String)>) {}

async fn edit_chapter(Path((book_id, chapter_id)): Path<(String, String)>) {}

async fn download_book(Path(book_id): Path<String>) {}

async fn search_book() {}
