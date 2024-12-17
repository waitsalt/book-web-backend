use axum::Json;

use crate::{
    model::book::{BookInfo, BookSearchPayload},
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn book_search(
    Json(search_book_payload): Json<BookSearchPayload>,
) -> AppResult<Vec<BookInfo>> {
    let pool = get_pool().await;
    let books = sql::book::search_book(
        pool,
        &search_book_payload.book_name,
        &search_book_payload.author_name,
        &search_book_payload.platform,
    )
    .await?;
    Ok(AppResponse::success(Some(books)))
}
