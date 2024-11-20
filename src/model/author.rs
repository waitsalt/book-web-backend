use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Author {
    pub author_id: i32,
    pub author_name: String,
    pub platform: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}
