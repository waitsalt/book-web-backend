use axum::Json;

use crate::{
    model::user::{UserClaims, UserVerifyEmailPayload},
    sql,
    util::{self, app_response::AppResponse, auth::check_user, database::get_pool, AppResult},
};

pub async fn verify_email(
    user_claims_opt: Option<UserClaims>,
    Json(user_verify_email_payload): Json<UserVerifyEmailPayload>,
) -> AppResult<()> {
    let _user_claims = check_user(user_claims_opt).await?;
    let pool = get_pool().await;
    let _ = sql::user::query_user_email_is_exist(pool, &user_verify_email_payload.user_email).await;
    let _ = util::captcha_email::captcha_email(&user_verify_email_payload.user_email).await?;
    Ok(AppResponse::success(None))
}
