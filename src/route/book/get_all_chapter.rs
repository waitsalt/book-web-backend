use axum::extract::Path;

use crate::{
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn get_all_chapter(Path(book_id): Path<i32>) -> AppResult<Vec<(i32, String)>> {
    let pool = get_pool().await;
    let chapter_info_list = sql::chapter::get_all_chapter(pool, &book_id).await?;
    Ok(AppResponse::success((Some(chapter_info_list))))
}
