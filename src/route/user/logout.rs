use crate::{
    model::user::ClaimsUser,
    util::{app_response::AppResponse, redis::get_redis_connect, AppResult},
};

pub async fn logout(user: ClaimsUser) -> AppResult<Option<String>> {
    let mut con = get_redis_connect().await;

    let user_id = user.user_id;

    let token_key = format!("token:{}", user_id);

    let _: () = redis::cmd("DEL").arg(token_key).query(&mut con).unwrap();

    Ok(AppResponse::from("logout success".to_string(), None))
}
