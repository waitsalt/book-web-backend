use axum::{extract::Path, Json};

use crate::{
    model::{chapter::ChapterCreatePayload, user::UserClaims},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check_admin, database::get_pool,
        AppResult,
    },
};

pub async fn create_chapter(
    user_claims_opt: Option<UserClaims>,
    Path(book_id): Path<i32>,
    Json(chapter_create_payload): Json<ChapterCreatePayload>,
) -> AppResult<String> {
    let user_claims = check_admin(user_claims_opt).await?;
    let pool = get_pool().await;

    // 获取书籍信息
    let book = sql::book::get_book_info_by_id(pool, &book_id).await?;

    // 章节存在则返回报错
    let res = sql::chapter::get_chapter(pool, &book_id, &chapter_create_payload.chapter_id).await;
    match res {
        Ok(_) => {
            return Err(AppError::ChapterExist);
        }
        Err(_) => {}
    }

    sql::chapter::create_chapter(
        pool,
        &book.book_id,
        &book.book_name,
        &book.author_id,
        &book.author_name,
        &book.platform,
        &user_claims.user_public.user_id,
        &user_claims.user_public.user_name,
        &chapter_create_payload.roll_id,
        &chapter_create_payload.roll_name,
        &chapter_create_payload.chapter_id,
        &chapter_create_payload.chapter_name,
        &chapter_create_payload.chapter_content,
    )
    .await?;
    Ok(AppResponse::success(None))
}
