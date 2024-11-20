use axum::{extract::Path, Json};

use crate::{
    model::{chapter::CreateChapterPayload, user::ClaimsUser},
    sql,
    util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn create_chapter(
    claims_user: ClaimsUser,
    Path((_book_id, chapter_id)): Path<(i32, i32)>,
    Json(create_chapter_payload): Json<CreateChapterPayload>,
) -> AppResult<String> {
    if claims_user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;

    let book = sql::book::get_book_info_by_id(pool, &create_chapter_payload.book_id).await?;

    sql::chapter::create_chapter(
        pool,
        &book.book_id,
        &book.book_name,
        &book.author_id,
        &book.author_name,
        &book.platform,
        &create_chapter_payload.roll_id,
        &create_chapter_payload.roll_name,
        &chapter_id,
        &create_chapter_payload.chapter_name,
        &create_chapter_payload.chapter_content,
    )
    .await?;
    Ok(AppResponse::success(None))
}
