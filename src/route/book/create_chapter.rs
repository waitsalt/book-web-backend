use axum::{extract::Path, Json};

use crate::{
    model::{chapter::CreateChapterPayload, user::ClaimsUser},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check_admin, database::get_pool,
        AppResult,
    },
};

pub async fn create_chapter(
    claims_user_opt: Option<ClaimsUser>,
    Path(book_id): Path<i32>,
    Json(create_chapter_payload): Json<CreateChapterPayload>,
) -> AppResult<String> {
    let _claims_user = check_admin(claims_user_opt).await?;
    let pool = get_pool().await;

    // 获取书籍信息
    let book = sql::book::get_book_info_by_id(pool, &book_id).await?;

    // 章节存在则返回报错
    let res =
        sql::chapter::get_chapter_info(pool, &book_id, &create_chapter_payload.chapter_id).await;
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
        &create_chapter_payload.roll_id,
        &create_chapter_payload.roll_name,
        &create_chapter_payload.chapter_id,
        &create_chapter_payload.chapter_name,
        &create_chapter_payload.chapter_content,
    )
    .await?;
    Ok(AppResponse::success(None))
}
