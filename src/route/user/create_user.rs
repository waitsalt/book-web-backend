use axum::Json;

use crate::{
    model::user::{ClaimsUser, SignupUserPayload},
    sql,
    util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn create_user(
    claims_user: ClaimsUser,
    Json(signup_user_payload): Json<SignupUserPayload>,
) -> AppResult<String> {
    if claims_user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;
    // 查询用户名是否存在
    sql::user::query_user_name_is_exist(pool, &signup_user_payload.user_name).await?;

    // 查询用户邮箱是否存在
    sql::user::query_user_email_is_exist(pool, &signup_user_payload.user_email).await?;

    // 检查密码
    if signup_user_payload.user_name.len() < 8 {
        return Err(AppError::UserPasswordShort);
    }

    // 新建用户
    sql::user::create_user(
        pool,
        &signup_user_payload.user_name,
        &signup_user_payload.user_password,
        &signup_user_payload.user_email,
        &signup_user_payload.avatar_url,
    )
    .await?;

    Ok(AppResponse::success(None))
}
