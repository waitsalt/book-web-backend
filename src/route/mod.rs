mod book;
mod user;
mod util;

use axum::Router;
use tower_http::{
    cors::{AllowHeaders, Any, CorsLayer},
    trace,
};

use crate::util::{app_error::AppError, AppResult};

pub async fn init() -> Router {
    let book_router = book::init().await;
    let user_router = user::init().await;
    let util_router = util::init().await;
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_headers(AllowHeaders::list(vec![
            "Authorization".parse().unwrap(),
            "Content-Type".parse().unwrap(),
            "Accept".parse().unwrap(),
        ]));
    Router::new()
        .nest("/api/book", book_router)
        .nest("/api/user", user_router)
        .nest("/api/util", util_router)
        .fallback(fallback)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}

async fn fallback() -> AppResult<String> {
    Err(AppError::NotFound)
}
