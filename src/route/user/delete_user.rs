use axum::extract::Path;

use crate::util::{
    app_error::AppError, app_response::AppResponse, auth::ClaimsUser, database::get_pool, AppResult,
};

pub async fn delete_user(user: ClaimsUser, Path(user_id): Path<String>) -> AppResult<u64> {
    if user.identity < 2 {
        return Err(AppError::UserMissPermission);
    }
    let pool = get_pool().await;

    let sql = "
    upadte \"user\"
    set user_status = 2
    where user_id = $1
    ";
    let affected_row = sqlx::query(sql)
        .bind(&user_id)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();

    Ok(AppResponse::success(Some(affected_row)))
}
