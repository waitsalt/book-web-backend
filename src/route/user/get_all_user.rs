use crate::{
    model::user::User,
    util::{
        app_error::AppError, app_response::AppResponse, auth::ClaimsUser, database::get_pool,
        AppResult,
    },
};

pub async fn get_all_user(user: ClaimsUser) -> AppResult<Vec<User>> {
    if user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;

    let sql = "
    select * from
        \"user\"
    ";

    let users: Vec<User> = sqlx::query_as(sql).fetch_all(pool).await?;

    Ok(AppResponse::success(Some(users)))
}
