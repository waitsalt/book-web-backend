use axum::{routing::get, Router};

pub async fn init() -> Router {
    Router::new().route("/", get(book_root))
}

async fn book_root() -> &'static str {
    "this is api book root path"
}
