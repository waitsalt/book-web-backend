use axum::Json;

use crate::{
    model::{
        book::{BookCreateInfoPayload, BookCreatePayload},
        user::UserClaims,
    },
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check_admin, database::get_pool,
        AppResult,
    },
};

pub async fn create_book(
    user_claims_opt: Option<UserClaims>,
    Json(book_create_payload): Json<BookCreatePayload>,
) -> AppResult<String> {
    let user_claims = check_admin(user_claims_opt).await?;

    let pool = get_pool().await;
    let book_create_info_payload = book_create_payload.book_create_info_payload;

    // 确认作者是否存在
    let res = sql::author::get_author_info(
        pool,
        &book_create_info_payload.author_name,
        &book_create_info_payload.platform,
    )
    .await;

    let author = match res {
        Ok(author) => author,
        Err(_) => {
            // 新建
            sql::author::create_author(
                pool,
                &book_create_info_payload.author_name,
                &book_create_info_payload.platform,
            )
            .await?;
            // 获取
            sql::author::get_author_info(
                pool,
                &book_create_info_payload.author_name,
                &book_create_info_payload.platform,
            )
            .await?
        }
    };

    let res = sql::book::get_book_info_by_book_name_with_author_id(
        pool,
        &book_create_info_payload.book_name,
        &author.author_id,
    )
    .await;
    match res {
        Ok(_) => return Err(AppError::BookExist),
        Err(_) => (),
    }
    sql::book::create_book(
        pool,
        &book_create_info_payload.book_name,
        &author.author_id,
        &author.author_name,
        &author.platform,
        &user_claims.user_public.user_id,
        &user_claims.user_public.user_name,
        &book_create_info_payload.cover_url,
        &book_create_info_payload.source_url,
        &book_create_info_payload.book_tags,
        &book_create_info_payload.book_desc,
        &book_create_info_payload.book_class,
        &book_create_info_payload.book_status,
    )
    .await?;

    let book = sql::book::get_book_info_by_book_name_with_author_id(
        pool,
        &book_create_info_payload.book_name,
        &author.author_id,
    )
    .await?;

    let chapter_content = format!(
        "书名:{}\n作者:{}\n平台:{}\n来源:{}\n状态:{}\n类别:{}\n标签:{}\n简介:{}",
        book_create_info_payload.book_name,
        book_create_info_payload.author_name,
        book_create_info_payload.platform,
        book_create_info_payload.source_url,
        book_create_info_payload.book_status,
        book_create_info_payload.book_class,
        book_create_info_payload.book_tags,
        book_create_info_payload.book_desc,
    );

    for (roll_id, roll_create) in book_create_payload.roll_create_list.iter().enumerate() {
        let roll_id = roll_id as i16;
        let _ = sql::roll::create_roll(
            pool,
            &book.book_id,
            &book.book_name,
            &book.author_id,
            &book.author_name,
            &book.platform,
            &roll_id,
            &roll_create.roll_name,
        )
        .await
        .unwrap();
        for (chapter_id, chapter_create) in roll_create.chapter_create_list.iter().enumerate() {
            let chapter_id = chapter_id as i32;
            let _ = sql::chapter::create_chapter(
                &pool,
                &book.book_id,
                &book.book_name,
                &book.author_id,
                &book.author_name,
                &book.platform,
                &user_claims.user_public.user_id,
                &user_claims.user_public.user_name,
                &roll_id,
                &roll_create.roll_name,
                &chapter_id,
                &chapter_create.chapter_name,
                &chapter_create.chapter_content,
            )
            .await
            .unwrap();
        }
    }
    Ok(AppResponse::success(None))
}
