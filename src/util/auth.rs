use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};

use crate::model::user::{UserClaims, UserRefreshClaims};

use super::{app_error::AppError, config::CONFIG};

pub async fn sign(user_claims: UserClaims) -> Result<String, AppError> {
    let secret = CONFIG.auth.secret.clone();
    let token = jsonwebtoken::encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Ok(token)
}

pub async fn refresh_sign(user_refresh_claims: UserRefreshClaims) -> Result<String, AppError> {
    let secret = CONFIG.auth.secret.clone();
    let token = jsonwebtoken::encode(
        &Header::default(),
        &user_refresh_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Ok(token)
}

pub async fn check_user_status(user_claims: UserClaims) -> Result<UserClaims, AppError> {
    let local_time = Utc::now().timestamp_millis();
    if local_time > user_claims.exp {
        return Err(AppError::AccessTokenMiss);
    }
    match user_claims.user_public.status {
        0 => Ok(user_claims),
        1 => Err(AppError::UserBlocked),
        2 => Err(AppError::UserDeleted),
        _ => Err(AppError::Other),
    }
}

pub async fn check_user(user_claims_opt: Option<UserClaims>) -> Result<UserClaims, AppError> {
    match user_claims_opt {
        Some(user_claims) => check_user_status(user_claims).await,
        None => {
            return Err(AppError::AccessTokenMiss);
        }
    }
}

pub async fn check_admin(user_claims_opt: Option<UserClaims>) -> Result<UserClaims, AppError> {
    match user_claims_opt {
        Some(user_claims) => {
            if user_claims.user_public.identity < 1 {
                return Err(AppError::UserMissPermission);
            }
            check_user_status(user_claims).await
        }
        None => {
            return Err(AppError::AccessTokenMiss);
        }
    }
}

pub async fn check_super_admin(
    user_claims_opt: Option<UserClaims>,
) -> Result<UserClaims, AppError> {
    match user_claims_opt {
        Some(user_claims) => {
            if user_claims.user_public.identity < 2 {
                return Err(AppError::UserMissPermission);
            }
            check_user_status(user_claims).await
        }
        None => {
            return Err(AppError::AccessTokenMiss);
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
            None => Err(AppError::AccessTokenInvalid),
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
            None => Err(AppError::RefreshTokenInvalid),
        }
    }
}
