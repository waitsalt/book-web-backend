use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Chapter {
    pub book_id: i32,
    pub book_name: String,
    pub author_id: i32,
    pub author_name: String,
    pub platform: String,
    pub uploader_id: i32,
    pub uploader_name: String,
    pub roll_id: i16,
    pub roll_name: String,
    pub chapter_id: i32,
    pub chapter_name: String,
    pub chapter_content: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ChapterInfo {
    pub book_id: i32,
    pub book_name: String,
    pub author_id: i32,
    pub author_name: String,
    pub platform: String,
    pub uploader_id: i32,
    pub uploader_name: String,
    pub roll_id: i16,
    pub roll_name: String,
    pub chapter_id: i32,
    pub chapter_name: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterCreatePayload {
    pub book_name: String,
    pub roll_id: i16,
    pub roll_name: String,
    pub chapter_id: i32,
    pub chapter_name: String,
    pub chapter_content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterCreate {
    pub chapter_name: String,
    pub chapter_content: String,
}
