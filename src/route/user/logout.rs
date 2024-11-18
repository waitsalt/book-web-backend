use crate::util::{app_response::AppResponse, auth::ClaimsUser, redis::get_client, AppResult};

pub async fn logout(user: ClaimsUser) -> AppResult<Option<String>> {
    let client = get_client().await;
    let mut con = client.get_connection().unwrap();

    let user_id = user.user_id;

    let token_key = format!("token:{}", user_id);

    let _: () = redis::cmd("DEL").arg(token_key).query(&mut con).unwrap();

    Ok(AppResponse::from("logout success".to_string(), None))
}
