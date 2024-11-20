use axum::Json;

use crate::{
    model::{book::CreateBookPayload, user::ClaimsUser},
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn create_book(
    claims_user: ClaimsUser,
    Json(create_book_payload): Json<CreateBookPayload>,
) -> AppResult<String> {
    let pool = get_pool().await;

    // 确认作者是否存在
    let res = sql::author::query_author_is_exist(
        pool,
        &create_book_payload.author_name,
        &create_book_payload.platform,
    )
    .await;
    match res {
        Ok(_) => {}
        // 不存在则新建
        Err(_) => {
            sql::author::create_author(
                pool,
                &create_book_payload.author_name,
                &create_book_payload.platform,
            )
            .await?
        }
    }
    let author = sql::author::get_author_info(
        pool,
        &create_book_payload.author_name,
        &create_book_payload.platform,
    )
    .await?;
    sql::book::create_book(
        pool,
        &create_book_payload.book_name,
        &author.author_id,
        &author.author_name,
        &author.platform,
        &claims_user.user_id,
        &claims_user.user_name,
        &create_book_payload.cover_url,
        &create_book_payload.source_url,
        &create_book_payload.book_tags,
        &create_book_payload.book_desc,
        &create_book_payload.book_class,
        &create_book_payload.book_status,
    )
    .await?;
    Ok(AppResponse::success(None))
}
