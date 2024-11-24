use axum::extract::Path;

use crate::{
    model::user::{ClaimsUser, PublicUser},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::check_user, database::get_pool,
        AppResult,
    },
};

pub async fn query_user_info(claims_user_opt: Option<ClaimsUser>) -> AppResult<PublicUser> {
    let claims_user = check_user(claims_user_opt).await?;
    let pool = get_pool().await;

    let user = sql::user::get_user_info_by_id(pool, &claims_user.user_id).await?;
    match user.status {
        0 => {
            return Ok(AppResponse::success(Some(PublicUser::from(user))));
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
