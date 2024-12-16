use sqlx::{Pool, Postgres};

use crate::util::{app_response::AppResponse, AppResult};

pub async fn create_roll(
    pool: &Pool<Postgres>,
    book_id: &i32,
    book_name: &str,
    author_id: &i32,
    author_name: &str,
    platform: &str,
    roll_id: &i16,
    roll_name: &str,
) -> AppResult<()> {
    let sql = "
        insert into
            roll (book_id,book_name,author_id,author_name,platform,roll_id,roll_name)
        values
            ($1,$2,$3,$4,$5,$6,$7)";
    let _ = sqlx::query(sql)
        .bind(book_id)
        .bind(book_name)
        .bind(author_id)
        .bind(author_name)
        .bind(platform)
        .bind(roll_id)
        .bind(roll_name)
        .execute(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(None))
}
