mod captcha_email;
mod captcha_image;

use axum::{routing::get, Router};

pub async fn init() -> Router {
    Router::new()
        .route(
            "/captcha_email/:user_email",
            get(captcha_email::captcha_email),
        )
        .route(
            "/captcha_image/:captcha_image_key",
            get(captcha_image::captcha_image),
        )
}
