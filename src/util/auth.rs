use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};

use crate::model::user::{UserClaims, UserRefreshClaims};

use super::{app_error::AppError, config::CONFIG};

pub async fn sign(claims_user: UserClaims) -> Result<String, AppError> {
    let secret = CONFIG.auth.secret.clone();
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims_user,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Ok(token)
}

pub async fn refresh_sign(refresh_claims_user: UserRefreshClaims) -> Result<String, AppError> {
    let secret = CONFIG.auth.secret.clone();
    let token = jsonwebtoken::encode(
        &Header::default(),
        &refresh_claims_user,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Ok(token)
}

pub async fn check_user_status(claims_user: UserClaims) -> Result<UserClaims, AppError> {
    let local_time = Utc::now().timestamp();
    if local_time < claims_user.exp {
        return Err(AppError::TokenInvalid);
    }
    match claims_user.user_info.status {
        0 => {
            return Ok(claims_user);
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
    }
}

pub async fn check_super_admin(
    claims_user_opt: Option<UserClaims>,
) -> Result<UserClaims, AppError> {
    match claims_user_opt {
        Some(claims_user) => check_user_status(claims_user).await,
        None => {
            return Err(AppError::TokenMiss);
        }
    }
}

pub async fn check_admin(claims_user_opt: Option<UserClaims>) -> Result<UserClaims, AppError> {
    match claims_user_opt {
        Some(claims_user) => check_user_status(claims_user).await,
        None => {
            return Err(AppError::TokenMiss);
        }
    }
}

pub async fn check_user(claims_user_opt: Option<UserClaims>) -> Result<UserClaims, AppError> {
    match claims_user_opt {
        Some(claims_user) => check_user_status(claims_user).await,
        None => {
            return Err(AppError::TokenMiss);
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserClaims
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
                let token_data = decode::<UserClaims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                )
                .unwrap();
                Ok(token_data.claims)
            }
            None => Err(AppError::TokenInvalid),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserRefreshClaims
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
                let token_data = decode::<UserRefreshClaims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                )
                .unwrap();
                Ok(token_data.claims)
            }
            None => Err(AppError::TokenInvalid),
        }
    }
}
