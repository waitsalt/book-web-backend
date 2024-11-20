use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    model::user::{ClaimsUser, User},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::sign, config::CONFIG,
        database::get_pool, redis::get_client, AppResult,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUserPayload {
    pub user_name: String,
    pub user_password: String,
}

pub async fn signin(Json(auth_user_payload): Json<AuthUserPayload>) -> AppResult<String> {
    let pool = get_pool().await;

    let user = sql::user::get_user_info_by_name(pool, &auth_user_payload.user_name).await?;

    match user.status {
        0 => {
            if user.user_password != auth_user_payload.user_password {
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

async fn auth(user: User) -> AppResult<String> {
    let client = get_client().await;
    let mut con = client.get_connection().unwrap();

    let token_key = format!("token:{}", user.user_id);

    let login_token_string: String = match redis::cmd("GET").arg(token_key.clone()).query(&mut con)
    {
        Ok(login_token) => login_token,
        Err(_) => "".to_string(),
    };

    if !login_token_string.is_empty() {
        Ok(AppResponse::success(Some(login_token_string)))
    } else {
        let duration = CONFIG.auth.duration;
        let token = sign(ClaimsUser::from(user)).await?;

        let _: () = redis::cmd("SET")
            .arg(token_key)
            .arg(token.clone())
            .arg("EX")
            .arg(duration * 3600)
            .query(&mut con)
            .unwrap();
        Ok(AppResponse::success(Some(token)))
    }
}
