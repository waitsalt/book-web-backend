use axum::Json;

use crate::{
    model::book::{Book, SearchBookPayload},
    sql::book::{
        search_book_by_author_name, search_book_by_book_name,
        search_book_by_book_name_or_author_name,
    },
    util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn search_book(
    Json(search_book_payload): Json<SearchBookPayload>,
) -> AppResult<Vec<Book>> {
    let pool = get_pool().await;
    match search_book_payload.sort.as_str() {
        "book_name" => {
            let books = search_book_by_book_name(pool, &search_book_payload.keyword).await?;
            Ok(AppResponse::success(Some(books)))
        }
        "author_name" => {
            let books = search_book_by_author_name(pool, &search_book_payload.keyword).await?;
            Ok(AppResponse::success(Some(books)))
        }
        "all" => {
            let books =
                search_book_by_book_name_or_author_name(pool, &search_book_payload.keyword).await?;
            Ok(AppResponse::success(Some(books)))
        }
        _ => {
            return Err(AppError::Other);
        }
    }
}
