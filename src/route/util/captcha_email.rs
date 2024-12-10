use axum::extract::Path;

use crate::{
    sql,
    util::{self, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn captcha_email(Path(user_email): Path<String>) -> AppResult<()> {
    let pool = get_pool().await;
    let _ = sql::user::query_user_email_is_exist(pool, &user_email).await?;
    let _ = util::captcha_email::captcha_email(&user_email).await?;
    Ok(AppResponse::success(None))
}
