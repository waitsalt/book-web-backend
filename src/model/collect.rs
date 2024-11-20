use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Collect {
    pub user_id: i32,
    pub create_time: DateTime<Utc>,
}
