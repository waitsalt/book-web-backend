use axum::Json;

use crate::{
    model::user::{UserClaims, UserUpdatePasswordPayload},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check_user, database::get_pool,
        AppResult,
    },
};

pub async fn update_password(
    user_claims_opt: Option<UserClaims>,
    Json(user_update_password_payload): Json<UserUpdatePasswordPayload>,
) -> AppResult<()> {
    let user_claims = check_user(user_claims_opt).await?;
    let pool = get_pool().await;
    let user = sql::user::get_user_info_by_id(pool, &user_claims.user_public.user_id)
        .await
        .unwrap();
    if user.user_password != user_update_password_payload.old_password {
        return Err(AppError::UserPasswordError);
    }
    let _ = sql::user::update_password(
        pool,
        &user_claims.user_public.user_id,
        &user_update_password_payload.new_password,
    )
    .await
    .unwrap();
    Ok(AppResponse::success(None))
}
