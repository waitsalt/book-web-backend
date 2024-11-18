use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    model::user::User,
    util::{
        app_error::AppError,
        app_response::AppResponse,
        auth::{sign, ClaimsUser},
        config::CONFIG,
        database::get_pool,
        redis::get_client,
        AppResult,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUserPayload {
    pub user_name: String,
    pub user_password: String,
}

pub async fn signin(Json(auth_user_payload): Json<AuthUserPayload>) -> AppResult<String> {
    let pool = get_pool().await;
    let sql = "
        select
            *
        from 
           \"user\"
        where
            user_name = $1
        and
            user_password = $2;";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(&auth_user_payload.user_name)
        .bind(&auth_user_payload.user_password)
        .fetch_optional(pool)
        .await?;
    match res {
        Some(user) => match user.status {
            0 => auth(user).await,
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
