use axum::Json;

use crate::{
    component::model::book::{Book, DescBook},
    util::{database::POOL, error::AppError},
};

pub async fn update_book_info_db(book_info: &Book) -> Result<(), AppError> {
    let pool = POOL.get().expect("pool get error").clone();
    let res = sqlx::query(
        "insert into public.book(id,name,author,status,tag,\"desc\",chapter) values ($1,$2,$3,$4,$5,$6,$7);",
    )
    .bind(&book_info.id)
    .bind(&book_info.name)
    .bind(&book_info.author)
    .bind(&book_info.status)
    .bind(&book_info.tag)
    .bind(&book_info.desc)
    .bind(&book_info.chapter)
    .fetch_optional(&pool)
    .await
    .unwrap();
    Ok(())
}

pub async fn fuzzy_search_book_by_name_and_author(
    name: &String,
    author: &String,
) -> Result<Json<Vec<DescBook>>, AppError> {
    if name.len() + author.len() == 0 {
        return Err(AppError::Other);
    }
    let name = format!("%{}%", name);
    let author = format!("%{}%", author);
    let pool = POOL.get().expect("pool get error").clone();
    let books: Vec<DescBook> = sqlx::query_as(
        "select id,name,author,status,tag,\"desc\" from public.book where name like $1 and author like $2;",
    )
    .bind(&name)
    .bind(&author)
    .fetch_all(&pool)
    .await
    .unwrap();
    Ok(Json(books))
}

pub async fn check_book_unique(name: &String, author: &String) -> Result<bool, AppError> {
    let pool = POOL.get().unwrap().clone();
    let res = sqlx::query("select id from public.book where name = $1 and author = $2;")
        .bind(&name)
        .bind(&author)
        .fetch_optional(&pool)
        .await
        .unwrap();
    match res {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
