use crate::{
    model::user::UserClaims,
    util::{app_response::AppResponse, auth::check_user, AppResult},
};

pub async fn verify_access_token(user_claims_opt: Option<UserClaims>) -> AppResult<UserClaims> {
    let user_claims = check_user(user_claims_opt).await?;
    return Ok(AppResponse::success(Some(user_claims)));
}
