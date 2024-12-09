use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct BookInfo {
    pub book_id: i32,
    pub book_name: String,
    pub author_id: i32,
    pub author_name: String,
    pub platform: String,
    pub user_id: i32,
    pub user_name: String,
    pub manager_id: i32,
    pub manager_name: String,
    pub cover_url: String,
    pub source_url: String,
    pub book_tags: String,
    pub book_desc: String,
    pub book_class: String,
    pub book_status: String,
    pub latest_chapter_id: i32,
    pub latest_chapter_name: String,
    pub collect: i32,
    pub recommend: i32,
    pub read_time: i32,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookCreatePayload {
    pub book_name: String,
    pub author_name: String,
    pub platform: String,
    pub cover_url: String,
    pub source_url: String,
    pub book_tags: String,
    pub book_desc: String,
    pub book_class: String,
    pub book_status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookSearchPayload {
    pub book_name: String,
    pub author_name: String,
    pub platform: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookVerifyPayload {
    pub book_name: String,
    pub author_name: String,
    pub platform: String,
}
