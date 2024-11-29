use axum::Json;

use crate::{
    model::user::UserSignupPayload,
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, database::get_pool,
        redis::get_redis_connect, AppResult,
    },
};

pub async fn signup(Json(create_user_payload): Json<UserSignupPayload>) -> AppResult<String> {
    let pool = get_pool().await;
    // 查询用户名是否存在
    sql::user::query_user_name_is_exist(pool, &create_user_payload.user_name).await?;

    // 查询用户邮箱是否存在
    sql::user::query_user_email_is_exist(pool, &create_user_payload.user_email).await?;

    // 检查密码
    if create_user_payload.user_password.len() < 8 {
        return Err(AppError::UserPasswordShort);
    }

    // 验证邮箱验证码
    let mut con = get_redis_connect().await;

    let captcha_email_key = format!("captcha_email_key:{}", create_user_payload.user_email);
    let captcha_email_string: String = match redis::cmd("GET")
        .arg(captcha_email_key.clone())
        .query(&mut con)
    {
        Ok(captcha_email) => captcha_email,
        Err(_) => "".to_string(),
    };

    if captcha_email_string != create_user_payload.captcha_email {
        return Err(AppError::CaptchaEmailError);
    }

    // 新建用户
    sql::user::create_user(
        pool,
        &create_user_payload.user_name,
        &create_user_payload.user_password,
        &create_user_payload.user_email,
        &create_user_payload.avatar_url,
    )
    .await?;

    // 验证码用户建立成功后失效
    let _: () = redis::cmd("DEL")
        .arg(captcha_email_key)
        .query(&mut con)
        .unwrap();

    Ok(AppResponse::success(None))
}
