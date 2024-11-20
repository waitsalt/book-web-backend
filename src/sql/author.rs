use sqlx::{Pool, Postgres};

use crate::{
    model::author::Author,
    util::{app_error::AppError, Result},
};

pub async fn query_author_is_exist(
    pool: &Pool<Postgres>,
    author_name: &str,
    platform: &str,
) -> Result<()> {
    let sql = "
        select
            author_id, author_name
        from
            author
        where
            author_name = $1
        and
            platform = $2;";
    let res: Option<(i32, i32)> = sqlx::query_as(sql)
        .bind(author_name)
        .bind(platform)
        .fetch_optional(pool)
        .await?;
    match res {
        Some(_) => {}
        None => return Err(AppError::AuthorNotExist),
    }
    Ok(())
}

pub async fn create_author(pool: &Pool<Postgres>, author_name: &str, platform: &str) -> Result<()> {
    let sql = "
        insert into
            author (author_name, platform)
        values
            ($1, $2)";
    let _ = sqlx::query(sql)
        .bind(author_name)
        .bind(platform)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn get_author_info(
    pool: &Pool<Postgres>,
    author_name: &str,
    platform: &str,
) -> Result<Author> {
    let sql = "
    select
        *
    from
        author
    where
        author_name = $1
    and
        platform = $2;";
    let res: Option<Author> = sqlx::query_as(sql)
        .bind(author_name)
        .bind(platform)
        .fetch_optional(pool)
        .await?;
    match res {
        Some(author) => Ok(author),
        None => return Err(AppError::AuthorNotExist),
    }
}
