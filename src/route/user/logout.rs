use crate::{
    model::user::UserRefreshClaims,
    util::{app_response::AppResponse, redis::get_redis_connect, AppResult},
};

pub async fn logout(user_refresh_claims: UserRefreshClaims) -> AppResult<()> {
    let mut con = get_redis_connect().await;

    println!("{:?}", user_refresh_claims);

    let user_id = user_refresh_claims.user_id;

    let token_key = format!("refresh_token:{}", user_id);

    let _: () = redis::cmd("DEL").arg(token_key).query(&mut con).unwrap();

    Ok(AppResponse::success(None))
}
