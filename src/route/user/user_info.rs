use axum::extract::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    model::user::User,
    util::{
        app_error::AppError, app_response::AppResponse, auth::ClaimsUser, database::get_pool,
        AppResult,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicUser {
    pub user_id: String,
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
    fn from(user: User) -> Self {
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

pub async fn user_info(user: ClaimsUser, Path(user_id): Path<String>) -> AppResult<PublicUser> {
    if user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;
    let sql = "
    select
        *
    from 
        \"user\"
    where
        user_id = $1;";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(&user_id)
        .fetch_optional(pool)
        .await?;
    match res {
        Some(user) => match user.status {
            0 => {
                return Ok(AppResponse::success(Some(PublicUser::from(user))));
            }
            1 => {
                return Err(AppError::UserBlocked);
            }
            2 => {
                return Err(AppError::UserDeleted);
            }
            _ => {
                return Err(AppError::Other);
            }
        },
        None => {
            return Err(AppError::UserNotExist);
        }
    }
}
