use axum::extract::Path;
use axum::Json;

use crate::model::book::Book;
use crate::util::database::POOL;
use crate::util::error::{AppError, BookError};

pub async fn book_info(Path(book_id): Path<String>) -> Result<Json<Book>, AppError> {
    let pool = POOL.get().unwrap().clone();
    let res: Option<Book> = sqlx::query_as("select * from public.book where book_id = $1")
        .bind(book_id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    match res {
        Some(book) => {
            return Ok(Json(book));
        }
        None => {
            return Err(AppError::BookError(BookError::BookNotExist));
        }
    }
}
