use chrono::Utc;

use crate::{
    model::user::{UserClaims, UserRefreshClaims},
    sql,
    util::{
        app_error::AppError, app_response::AppResponse, auth::sign, database::get_pool, AppResult,
    },
};

pub async fn refresh_access_token(
    user_refresh_claims_opt: Option<UserRefreshClaims>,
) -> AppResult<String> {
    let user_refresh_claims = match user_refresh_claims_opt {
        Some(user_refresh_claims) => {
            let local_time = Utc::now().timestamp();
            if local_time > user_refresh_claims.exp {
                return Err(AppError::RefreshTokenInvalid);
            }
            user_refresh_claims
        }
        None => {
            return Err(AppError::RefreshTokenMiss);
        }
    };
    let pool = get_pool().await;
    let user_id = user_refresh_claims.user_id;
    let user = sql::user::get_user_info_by_id(pool, &user_id).await?;
    let access_token = sign(UserClaims::from(user.clone())).await?;
    Ok(AppResponse::success(Some(access_token)))
}
