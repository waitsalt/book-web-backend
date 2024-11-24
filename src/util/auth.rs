use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};

use crate::model::user::ClaimsUser;

use super::{app_error::AppError, config::CONFIG};

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

pub async fn check_super_admin(
    claims_user_opt: Option<ClaimsUser>,
) -> Result<ClaimsUser, AppError> {
    match claims_user_opt {
        Some(claims_user) => {
            if claims_user.identity < 2 {
                return Err(AppError::UserMissPermission);
            }
            return Ok(claims_user);
        }
        None => {
            return Err(AppError::TokenMiss);
        }
    }
}

pub async fn check_admin(claims_user_opt: Option<ClaimsUser>) -> Result<ClaimsUser, AppError> {
    match claims_user_opt {
        Some(claims_user) => {
            if claims_user.identity < 1 {
                return Err(AppError::UserMissPermission);
            }
            return Ok(claims_user);
        }
        None => {
            return Err(AppError::TokenMiss);
        }
    }
}

pub async fn check_user(claims_user_opt: Option<ClaimsUser>) -> Result<ClaimsUser, AppError> {
    match claims_user_opt {
        Some(claims_user) => {
            return Ok(claims_user);
        }
        None => {
            return Err(AppError::TokenMiss);
        }
    }
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
            None => Err(AppError::TokenInvalid),
        }
    }
}
