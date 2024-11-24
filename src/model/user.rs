use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::util::config::CONFIG;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub user_id: i32,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SigninUserPayload {
    pub user_name: String,
    pub user_password: String,
    pub captcha_image_key: String,
    pub captcha_image: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserPayload {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub avatar_url: String,
    pub captcha_email: String,
    pub captcha_image_key: String,
    pub captcha_image: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicUser {
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub avatar_url: String, // 头像 url
    pub level: i16,         // 0
    pub status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

impl PublicUser {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        Self {
            user_id: user.user_id,
            user_name: user.user_name,
            user_email: user.user_email,
            avatar_url: user.avatar_url,
            level: user.level,
            status: user.status,
            identity: user.identity,
            create_time: user.create_time,
            update_time: user.update_time,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimsUser {
    pub exp: i64,
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub avatar_url: String, // 头像 url
    pub level: i16,         // 0
    pub status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
}

impl ClaimsUser {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        let duration = CONFIG.auth.duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::hours(duration);
        Self {
            exp: end_time.timestamp(),
            user_id: user.user_id,
            user_name: user.user_name,
            user_email: user.user_email,
            avatar_url: user.avatar_url,
            level: user.level,
            status: user.status,
            identity: user.identity,
        }
    }
}
