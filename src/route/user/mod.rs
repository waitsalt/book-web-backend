mod logout;
mod refresh_access_token;
mod signin;
mod signup;
mod update_avatar_url;
mod verify_access_token;

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
}
