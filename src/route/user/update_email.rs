use axum::Json;

use crate::{
    model::user::{UserClaims, UserUpdateEmailPayload},
    sql,
    util::{app_response::AppResponse, auth::check_user, database::get_pool, AppResult},
};

pub async fn update_email(
    user_claims_opt: Option<UserClaims>,
    Json(user_update_email_payload): Json<UserUpdateEmailPayload>,
) -> AppResult<()> {
    let user_claims = check_user(user_claims_opt).await?;
    let pool = get_pool().await;
    let _ =
        sql::user::query_user_email_is_exist(pool, &user_update_email_payload.user_email).await?;
    let _ = sql::user::update_email(
        pool,
        &user_claims.user_public.user_id,
        &user_update_email_payload.user_email,
    )
    .await
    .unwrap();
    Ok(AppResponse::success(None))
}
