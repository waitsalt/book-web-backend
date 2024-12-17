use axum::extract::Path;

use crate::{
    model::book::BookInfo,
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn book_info(Path(book_id): Path<i32>) -> AppResult<BookInfo> {
    let pool = get_pool().await;
    let book = sql::book::get_book_info_by_id(pool, &book_id).await?;
    Ok(AppResponse::success(Some(book)))
}
