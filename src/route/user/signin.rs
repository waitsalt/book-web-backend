use axum::Json;
use redis::Commands;

use crate::{
    model::user::{User, UserAuth, UserClaims, UserRefreshClaims, UserSigninPayload},
    sql,
    util::{
        app_error::AppError,
        app_response::AppResponse,
        auth::{refresh_sign, sign},
        config::CONFIG,
        database::get_pool,
        redis::get_redis_connect,
        AppResult,
    },
};

pub async fn signin(Json(signin_user_payload): Json<UserSigninPayload>) -> AppResult<UserAuth> {
    let mut con = get_redis_connect().await;

    let captcha_image_key = format!(
        "captcha_image_key:{}",
        signin_user_payload.captcha_image_key
    );
    let captcha_image: String = con.get_del(captcha_image_key).unwrap();

    if captcha_image != signin_user_payload.captcha_image {
        return Err(AppError::CaptchaImageError);
    }

    let pool = get_pool().await;

    let user = sql::user::get_user_info_by_name(pool, &signin_user_payload.user_name).await?;

    match user.status {
        0 => {
            if user.user_password != signin_user_payload.user_password {
                return Err(AppError::UserPasswordError);
            }
            auth(user).await
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

async fn auth(user: User) -> AppResult<UserAuth> {
    let mut con = get_redis_connect().await;

    let refresh_token_key = format!("refresh_token:{}", user.user_id);

    let refresh_token: String = match redis::cmd("GET")
        .arg(refresh_token_key.clone())
        .query(&mut con)
    {
        Ok(refresh_token) => refresh_token,
        Err(_) => "".to_string(),
    };

    let access_token = sign(UserClaims::from(user.clone())).await?;

    if !refresh_token.is_empty() {
        Ok(AppResponse::success(Some(UserAuth::new(
            access_token,
            refresh_token,
        ))))
    } else {
        let refresh_token_duration = CONFIG.auth.refresh_token_duration;
        let refresh_token = refresh_sign(UserRefreshClaims::from(user)).await?;

        let _: () = redis::cmd("SET")
            .arg(refresh_token_key)
            .arg(refresh_token.clone())
            .arg("EX")
            .arg(refresh_token_duration * 3600 * 24)
            .query(&mut con)
            .unwrap();
        Ok(AppResponse::success(Some(UserAuth::new(
            access_token,
            refresh_token,
        ))))
    }
}
