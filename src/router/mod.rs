mod book;
mod user;

use axum::Router;

pub async fn init() -> Router {
    let book_router = book::init().await;
    let user_router = user::init().await;
    Router::new()
        .nest("/api/book", book_router)
        .nest("/api/user", user_router)
}
