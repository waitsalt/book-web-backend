use axum::Json;

use crate::{
    model::user::{ClaimsUser, RefreshClaimsUser, SigninUserPayload, User},
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

pub async fn signin(
    Json(signin_user_payload): Json<SigninUserPayload>,
) -> AppResult<(String, String)> {
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

async fn auth(user: User) -> AppResult<(String, String)> {
    let mut con = get_redis_connect().await;

    let refresh_token_key = format!("refresh_token:{}", user.user_id);

    let refresh_token_string: String = match redis::cmd("GET")
        .arg(refresh_token_key.clone())
        .query(&mut con)
    {
        Ok(refresh_token) => refresh_token,
        Err(_) => "".to_string(),
    };

    let access_token = sign(ClaimsUser::from(user.clone())).await?;

    if !refresh_token_string.is_empty() {
        Ok(AppResponse::success(Some((
            refresh_token_key,
            access_token,
        ))))
    } else {
        let refresh_token_duration = CONFIG.auth.refresh_token_duration;
        let refresh_token = refresh_sign(RefreshClaimsUser::from(user)).await?;

        let _: () = redis::cmd("SET")
            .arg(refresh_token_key)
            .arg(refresh_token.clone())
            .arg("EX")
            .arg(refresh_token_duration * 3600 * 24)
            .query(&mut con)
            .unwrap();
        Ok(AppResponse::success(Some((refresh_token, access_token))))
    }
}
