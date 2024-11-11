use axum::{routing::get, Router};

pub async fn init() -> Router {
    Router::new().route("/", get(user_root))
}

async fn user_root() -> &'static str {
    "this is api user root path"
}
