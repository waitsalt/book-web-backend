use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct History {
    pub history_id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub book_id: i32,
    pub book_name: String,
    pub chapter_id: i32,
    pub chapter_name: String,
    pub create_time: DateTime<Utc>,
}
