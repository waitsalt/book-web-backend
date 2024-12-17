use axum::extract::Path;

use crate::{
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn chapter_info_list(Path(book_id): Path<i32>) -> AppResult<Vec<(i32, String)>> {
    let pool = get_pool().await;
    let chapter_info_list = sql::chapter::get_chapter_list(pool, &book_id).await?;
    Ok(AppResponse::success(Some(chapter_info_list)))
}
