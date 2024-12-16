use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::chapter::{Chapter, ChapterCreate};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Roll {
    pub book_id: i32,
    pub book_name: String,
    pub author_id: i32,
    pub author_name: String,
    pub platform: String,
    pub roll_id: i16,
    pub roll_name: String,
    pub chapter_list: Vec<Chapter>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RollCreate {
    pub roll_name: String,
    pub chapter_create_list: Vec<ChapterCreate>,
}
