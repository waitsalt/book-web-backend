use axum::{routing::get, Router};

pub async fn init() -> Router {
    Router::new().route("/", get(book))
}

pub async fn book() -> &'static str {
    "book"
}
