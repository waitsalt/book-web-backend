use axum::extract::Path;

use crate::model::user::ClaimsUser;
use crate::sql;
use crate::util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult};

pub async fn delete_user(
    claims_user: ClaimsUser,
    Path(user_id): Path<i32>,
) -> AppResult<Option<String>> {
    if claims_user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;

    sql::user::delete_user(pool, &user_id).await?;

    Ok(AppResponse::success(None))
}
