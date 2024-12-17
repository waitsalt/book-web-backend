use axum::extract::Path;

use crate::{
    model::chapter::Chapter,
    sql,
    util::{app_response::AppResponse, config::CONFIG, database::get_pool, AppResult},
};

pub async fn chapter(Path((book_id, chapter_id)): Path<(i32, i32)>) -> AppResult<Chapter> {
    let pool = get_pool().await;
    let chapter = sql::chapter::get_chapter(pool, &book_id, &chapter_id).await?;
    let book_base_path = CONFIG.data.path.clone();
    let _chapter_path = format!("{}/{}/{}", book_base_path, book_id, chapter_id);
    Ok(AppResponse::success(Some(chapter)))
}
