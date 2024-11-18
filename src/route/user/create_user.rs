use axum::Json;
use serde::{Deserialize, Serialize};

use crate::util::{
    app_error::AppError, app_response::AppResponse, auth::ClaimsUser, database::get_pool, AppResult,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupUser {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub avatar_url: String,
}

pub async fn create_user(
    user: ClaimsUser,
    Json(signup_user): Json<SignupUser>,
) -> AppResult<String> {
    if user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;
    // 查询用户名是否存在
    let sql = "
        select 
            user_id
        from
            \"user\"
        where
            user_name = $1
    ";
    let affected_row = sqlx::query(sql)
        .bind(&signup_user.user_name)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserNameExist);
    }

    // 查询用户名或用户邮箱是否存在
    let sql = "
        select 
            user_id
        from
            \"user\"
        where
            user_email = $1
    ";
    let affected_row = sqlx::query(sql)
        .bind(&signup_user.user_email)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserEmailExist);
    }

    // 检查密码
    if signup_user.user_name.len() < 8 {
        return Err(AppError::UserPasswordShort);
    }

    // 新建用户
    let user_id = nanoid::nanoid!();
    let sql = "
    insert into
        \"user\" (user_id, user_name, user_password, user_email, avatar_url)
    values
        ($1,$2,$3,$4,$5);
    ";
    let _affected_row = sqlx::query(sql)
        .bind(&user_id)
        .bind(&signup_user.user_name)
        .bind(&signup_user.user_password)
        .bind(&signup_user.user_email)
        .bind(&signup_user.avatar_url)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();

    Ok(AppResponse::success(None))
}
