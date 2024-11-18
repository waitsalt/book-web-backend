use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::model::user::User;

use super::{app_error::AppError, config::CONFIG};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimsUser {
    pub exp: i64,
    pub user_id: String,
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

pub async fn sign(claims_user: ClaimsUser) -> Result<String, AppError> {
    let secret = CONFIG.auth.secret.clone();
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims_user,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Ok(token)
}

#[async_trait]
impl<S> FromRequestParts<S> for ClaimsUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts<'life0, 'life1>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> Result<Self, Self::Rejection>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        let res = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(|token| token.to_string());
        match res {
            Some(token) => {
                let secret = CONFIG.auth.secret.clone();
                let token_data = decode::<ClaimsUser>(
                    &token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                )
                .unwrap();
                Ok(token_data.claims)
            }
            None => Err(AppError::InvalidToken),
        }
    }
}
