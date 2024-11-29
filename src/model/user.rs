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
pub struct UserSigninPayload {
    pub user_name: String,
    pub user_password: String,
    pub captcha_image_key: String,
    pub captcha_image: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSignupPayload {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub avatar_url: String,
    pub captcha_email: String,
    pub captcha_image_key: String,
    pub captcha_image: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserPublic {
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

impl UserPublic {
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
pub struct UserClaims {
    pub iat: i64,
    pub exp: i64,
    pub user_info: UserPublic,
}

impl UserClaims {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        let duration = CONFIG.auth.access_token_duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::minutes(duration);
        Self {
            iat: start_time.timestamp(),
            exp: end_time.timestamp(),
            user_info: UserPublic::from(user),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRefreshClaims {
    pub iat: i64,
    pub exp: i64,
    pub user_id: i32,
}

impl UserRefreshClaims {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        let duration = CONFIG.auth.refresh_token_duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::days(duration);
        Self {
            iat: start_time.timestamp(),
            exp: end_time.timestamp(),
            user_id: user.user_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuth {
    pub access_token: String,
    pub refresh_token: String,
}

impl UserAuth {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token: access_token,
            refresh_token: refresh_token,
        }
    }
}
