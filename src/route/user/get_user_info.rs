use axum::extract::Path;

use crate::{
    model::user::{ClaimsUser, PublicUser},
    sql,
    util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn query_user_info(
    claims_user: ClaimsUser,
    Path(user_id): Path<i32>,
) -> AppResult<PublicUser> {
    if claims_user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;

    let user = sql::user::get_user_info_by_id(pool, &user_id).await?;
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
