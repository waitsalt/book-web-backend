use axum::Json;

use crate::{
    model::{book::CreateBookPayload, user::UserClaims},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check_admin, database::get_pool,
        AppResult,
    },
};

pub async fn create_book(
    claims_user_opt: Option<UserClaims>,
    Json(create_book_payload): Json<CreateBookPayload>,
) -> AppResult<String> {
    let claims_user = check_admin(claims_user_opt).await?;

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
        &claims_user.user_info.user_id,
        &claims_user.user_info.user_name,
        &create_book_payload.cover_url,
        &create_book_payload.source_url,
        &create_book_payload.book_tags,
        &create_book_payload.book_desc,
        &create_book_payload.book_class,
        &create_book_payload.book_status,
    )
    .await?;

    let chapter_content = format!(
        "书名:{}\n作者:{}\n状态:{}\n类别:{}\n标签:{}\n简介:{}\n来源:{}",
        create_book_payload.book_name,
        create_book_payload.author_name,
        create_book_payload.book_status,
        create_book_payload.book_class,
        create_book_payload.book_tags,
        create_book_payload.book_desc,
        create_book_payload.source_url
    );

    let book = sql::book::get_book_info_by_book_name_with_author_id(
        pool,
        &create_book_payload.book_name,
        &author.author_id,
    )
    .await?;

    // 建立
    sql::chapter::create_chapter(
        pool,
        &book.book_id,
        &book.book_name,
        &book.author_id,
        &book.author_name,
        &book.platform,
        &0,
        "正文",
        &0,
        "书籍说明",
        &chapter_content,
    )
    .await?;
    Ok(AppResponse::success(None))
}
