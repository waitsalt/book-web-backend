use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, Json};
use serde_json::json;

use crate::util::database::POOL;
use crate::util::error::{AppError, BookError};

struct BookCreate {
    book_name: String,
    author_name: String,
    user_id: i32,
    user_name: String,
    book_picture_url: String,
    book_class: String,
    book_status: String,
    book_tags: String,
    book_desc: String,
}

pub async fn book_create(Json(book_create): Json<BookCreate>) -> Result<(), AppError> {
    let pool = POOL.get().unwrap().clone();

    let res = sqlx::query("select book_id from book where book_name = $1 and author_name = $2")
        .bind(&book_create.book_name)
        .bind(&book_create.author_name)
        .fetch_optional(&pool)
        .await
        .unwrap();

    match res {
        Some(_) => {
            return Err(AppError::BookError(BookError::BookExist));
        }
        None => {}
    }

    let res = sqlx::query("insert into book (book_name,author_name,user_id,user_name,book_picture_url,book_class,book_status,book_tags,book_desc) values ($1,$2,$3,$4,$5,$6,$7,$8,$9);")
        .bind(&book_create.book_name)
        .bind(&book_create.author_name)
        .bind(&book_create.user_id)
        .bind(&book_create.user_name)
        .bind(&book_create.book_picture_url)
        .bind(&book_create.book_class)
        .bind(&book_create.book_status)
        .bind(&book_create.book_tags)
        .bind(&book_create.book_desc)
        .fetch_optional(&pool).await.unwrap();
    match res {
        Some(data) => {
            println!("{:?}", data);
        }
        None => {}
    }
    Ok(())
}
