use axum::extract::Path;
use captcha_rust::Captcha;
use serde::{Deserialize, Serialize};

use crate::util::{app_response::AppResponse, AppResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaRes {
    key: String,
    image: String,
}

pub async fn captcha_image(Path(captcha_image_key): Path<String>) -> AppResult<String> {
    let a = Captcha::new(5, 130, 40);
    Ok(AppResponse::success(Some(a.base_img)))
}
