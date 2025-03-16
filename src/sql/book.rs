use sqlx::{Pool, Postgres};

use crate::{
    model::book::BookInfo,
    util::{app_error::AppError, Result},
};

pub async fn create_book(
    pool: &Pool<Postgres>,
    book_name: &str,
    author_id: &i32,
    author_name: &str,
    platform: &str,
    uploader_id: &i32,
    uploader_name: &str,
    cover_url: &str,
    source_url: &str,
    book_tags: &str,
    book_desc: &str,
    book_class: &str,
    book_status: &str,
) -> Result<()> {
    let sql = "
        insert into
            book (book_name, author_id, author_name, platform, uploader_id, uploader_name, cover_url, source_url, book_tags, book_desc, book_class, book_status)
        values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);";
    let _ = sqlx::query(sql)
        .bind(book_name)
        .bind(author_id)
        .bind(author_name)
        .bind(platform)
        .bind(uploader_id)
        .bind(uploader_name)
        .bind(cover_url)
        .bind(source_url)
        .bind(book_tags)
        .bind(book_desc)
        .bind(book_class)
        .bind(book_status)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn get_all_book(pool: &Pool<Postgres>) -> Result<Vec<BookInfo>> {
    let sql = "
    select
        *
    from
        book;";
    let books: Vec<BookInfo> = sqlx::query_as(sql).fetch_all(pool).await.unwrap();
    Ok(books)
}

pub async fn get_book_info_by_id(pool: &Pool<Postgres>, book_id: &i32) -> Result<BookInfo> {
    let sql = "
    select
        *
    from
        book
    where
        book_id = $1";
    let res: Option<BookInfo> = sqlx::query_as(sql)
        .bind(book_id)
        .fetch_optional(pool)
        .await
        .unwrap();
    match res {
        Some(book) => Ok(book),
        None => Err(AppError::BookNotExist),
    }
}

pub async fn get_book_info_by_book_name_with_author_id(
    pool: &Pool<Postgres>,
    book_name: &str,
    author_id: &i32,
) -> Result<BookInfo> {
    let sql = "
    select
        *
    from
        book
    where
        book_name = $1
    and
        author_id = $2;";
    let res: Option<BookInfo> = sqlx::query_as(sql)
        .bind(book_name)
        .bind(author_id)
        .fetch_optional(pool)
        .await
        .unwrap();
    match res {
        Some(book) => Ok(book),
        None => Err(AppError::BookNotExist),
    }
}

pub async fn search_book(
    pool: &Pool<Postgres>,
    book_name: &str,
    author_name: &str,
    platform: &str,
) -> Result<Vec<BookInfo>> {
    let book_name = format!("%{}%", book_name);
    let sql = "
        select
            *
        from
            book
        where
            book_name like $1
        and
            author_name like $2
        and
            platform like $3;";
    let books: Vec<BookInfo> = sqlx::query_as(sql)
        .bind(book_name)
        .bind(author_name)
        .bind(platform)
        .fetch_all(pool)
        .await
        .unwrap();
    Ok(books)
}

pub async fn verify_book(
    pool: &Pool<Postgres>,
    book_name: &str,
    author_name: &str,
    platform: &str,
) -> Result<Vec<BookInfo>> {
    let sql = "
        select
            *
        from
            book
        where
            book_name = $1
        and
            author_name = $2
        and
            platform = $3;";
    let books: Vec<BookInfo> = sqlx::query_as(sql)
        .bind(book_name)
        .bind(author_name)
        .bind(platform)
        .fetch_all(pool)
        .await
        .unwrap();
    Ok(books)
}

pub async fn book_list_latest_update(pool: &Pool<Postgres>) -> Result<Vec<BookInfo>> {
    let sql = "
        select *
        from book
        order by update_time desc
        limit 10;
        ";
    let book_info_list: Vec<BookInfo> = sqlx::query_as(sql).fetch_all(pool).await.unwrap();
    Ok(book_info_list)
}
