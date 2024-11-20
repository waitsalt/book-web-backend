use axum::Json;

use crate::{
    model::{book::CreateBookPayload, user::ClaimsUser},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check, database::get_pool, AppResult,
    },
};

pub async fn create_book(
    claims_user_opt: Option<ClaimsUser>,
    Json(create_book_payload): Json<CreateBookPayload>,
) -> AppResult<String> {
    let claims_user = check(claims_user_opt).await?;

    let pool = get_pool().await;

    // 确认作者是否存在
    let res = sql::author::get_author_info(
        pool,
        &create_book_payload.author_name,
        &create_book_payload.platform,
    )
    .await;

    let author = match res {
        Ok(author) => author,
        Err(_) => {
            // 新建
            sql::author::create_author(
                pool,
                &create_book_payload.author_name,
                &create_book_payload.platform,
            )
            .await?;
            // 获取
            sql::author::get_author_info(
                pool,
                &create_book_payload.author_name,
                &create_book_payload.platform,
            )
            .await?
        }
    };

    let res = sql::book::get_book_info_by_book_name_with_author_id(
        pool,
        &create_book_payload.book_name,
        &author.author_id,
    )
    .await;
    match res {
        Ok(_) => return Err(AppError::BookExist),
        Err(_) => (),
    }
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
