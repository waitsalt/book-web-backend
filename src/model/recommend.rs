use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Recommend {
    pub recommend_id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub book_id: i32,
    pub book_name: String,
    pub score: i16,
    pub create_time: DateTime<Utc>,
}
