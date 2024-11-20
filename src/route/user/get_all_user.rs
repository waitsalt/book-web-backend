use crate::{
    model::user::{ClaimsUser, User},
    sql,
    util::{app_error::AppError, app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn get_all_user(claims_user: ClaimsUser) -> AppResult<Vec<User>> {
    if claims_user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;

    let users = sql::user::get_all_user_info(pool).await?;

    Ok(AppResponse::success(Some(users)))
}
