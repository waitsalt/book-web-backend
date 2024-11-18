use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub avatar_url: String, // 头像 url
    pub level: i16,         // 0
    pub status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}
