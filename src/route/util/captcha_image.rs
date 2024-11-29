use axum::extract::Path;
use captcha_rust::Captcha;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::util::{app_response::AppResponse, redis::get_redis_connect, AppResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaRes {
    key: String,
    image: String,
}

pub async fn captcha_image(Path(captcha_image_key): Path<String>) -> AppResult<String> {
    let a = Captcha::new(5, 100, 40);
    let mut con = get_redis_connect().await;
    let captcha_image_key = format!("captcha_image_key:{}", captcha_image_key);
    let _: () = con.set_ex(captcha_image_key, a.text, 5 * 60).unwrap();
    Ok(AppResponse::success(Some(a.base_img)))
}
