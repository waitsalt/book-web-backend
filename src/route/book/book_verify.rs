use axum::Json;

use crate::{
    model::book::BookVerifyPayload,
    sql,
    util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn book_verify(Json(book_verify_payload): Json<BookVerifyPayload>) -> AppResult<()> {
    let pool = get_pool().await;
    let book_info_list = sql::book::verify_book(
        pool,
        &book_verify_payload.book_name,
        &book_verify_payload.author_name,
        &book_verify_payload.platform,
    )
    .await?;
    if book_info_list.len() != 0 {
        return Err(AppError::BookExist);
    }
    Ok(AppResponse::success(None))
}
