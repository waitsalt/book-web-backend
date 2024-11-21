use sqlx::{Pool, Postgres};

use crate::{
    model::chapter::Chapter,
    util::{app_error::AppError, Result},
};

pub async fn create_chapter(
    pool: &Pool<Postgres>,
    book_id: &i32,
    book_name: &str,
    author_id: &i32,
    author_name: &str,
    platform: &str,
    roll_id: &i16,
    roll_name: &str,
    chapter_id: &i32,
    chapter_name: &str,
    chapter_content: &str,
) -> Result<()> {
    let sql = "
        insert into
            chapter (book_id, book_name, author_id, author_name, platform, roll_id, roll_name, chapter_id, chapter_name, chapter_content)
        values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);";
    let _ = sqlx::query(sql)
        .bind(book_id)
        .bind(book_name)
        .bind(author_id)
        .bind(author_name)
        .bind(platform)
        .bind(roll_id)
        .bind(roll_name)
        .bind(chapter_id)
        .bind(chapter_name)
        .bind(chapter_content)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn get_all_chapter(pool: &Pool<Postgres>, book_id: &i32) -> Result<Vec<(i32, String)>> {
    let sql = "
        select
            chapter_id,chapter_name
        from
            chapter
        where
            book_id = $1;";
    let chapter_info_list: Vec<(i32, String)> = sqlx::query_as(sql)
        .bind(book_id)
        .fetch_all(pool)
        .await
        .unwrap();
    Ok(chapter_info_list)
}

pub async fn get_chapter_info(
    pool: &Pool<Postgres>,
    book_id: &i32,
    chapter_id: &i32,
) -> Result<Chapter> {
    let sql = "
        select
            *
        from
            chapter
        where
            book_id = $1
        and
            chapter_id = $2";
    let res: Option<Chapter> = sqlx::query_as(sql)
        .bind(book_id)
        .bind(chapter_id)
        .fetch_optional(pool)
        .await
        .unwrap();
    match res {
        Some(chapter) => Ok(chapter),
        None => Err(AppError::ChapterNotExist),
    }
}
