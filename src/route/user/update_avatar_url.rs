use axum::Json;

use crate::{
    model::user::{UserClaims, UserUpdateAvatarUrlPayload},
    sql,
    util::{app_response::AppResponse, auth::check_user, database::get_pool, AppResult},
};

pub async fn update_avatar_url(
    user_claims_opt: Option<UserClaims>,
    Json(user_update_avatar_url_payload): Json<UserUpdateAvatarUrlPayload>,
) -> AppResult<String> {
    let user_claims = check_user(user_claims_opt).await?;
    let pool = get_pool().await;
    let _ = sql::user::update_avatar_url(
        pool,
        &user_claims.user_public.user_id,
        &user_update_avatar_url_payload.avatar_url,
    )
    .await
    .unwrap();
    Ok(AppResponse::success(None))
}
