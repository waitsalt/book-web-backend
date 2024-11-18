mod create_user;
mod delete_user;
mod get_all_user;
mod logout;
mod signin;
mod user_info;

use axum::{
    routing::{get, post},
    Router,
};

pub async fn init() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_user::get_all_user).post(create_user::create_user),
        )
        .route(
            "/:user_id",
            get(user_info::user_info).delete(delete_user::delete_user),
        )
        .route("/signin", post(signin::signin))
        .route("/logout", get(logout::logout))
}