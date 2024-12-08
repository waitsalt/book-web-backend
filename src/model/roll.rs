use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::chapter::Chapter;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Rool {
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
