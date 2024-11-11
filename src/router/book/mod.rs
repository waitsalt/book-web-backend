mod book_create;
mod book_info;

use axum::{routing::get, Router};

pub async fn init() -> Router {
    Router::new()
        .route("/", get(book_root))
        .route("/:book_id", get(book_info::book_info))
}

async fn book_root() -> &'static str {
    "this is api book root path"
}
