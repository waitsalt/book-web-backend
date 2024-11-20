use axum::extract::Path;

use crate::{
    model::chapter::Chapter,
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn get_chapter_info(Path((book_id, chapter_id)): Path<(i32, i32)>) -> AppResult<Chapter> {
    let pool = get_pool().await;
    let chapter = sql::chapter::get_chapter_info(pool, &book_id, &chapter_id).await?;
    Ok(AppResponse::success(Some(chapter)))
}
