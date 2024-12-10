mod logout;
mod refresh_access_token;
mod signin;
mod signup;
mod update_avatar_url;
mod update_email;
mod update_password;
mod verify_access_token;
mod verify_email;

use axum::{
    routing::{get, post},
    Router,
};

pub async fn init() -> Router {
    Router::new()
        .route("/signup", post(signup::signup))
        .route("/signin", post(signin::signin))
        .route(
            "/refresh_access_token",
            get(refresh_access_token::refresh_access_token),
        )
        .route(
            "/update_avatar_url",
            post(update_avatar_url::update_avatar_url),
        )
        .route("/logout", get(logout::logout))
        .route(
            "/verify_access_token",
            get(verify_access_token::verify_access_token),
        )
        .route("/update_email", post(update_email::update_email))
        .route("/verify_email", post(verify_email::verify_email))
        .route("/update_password", post(update_password::update_password))
}
