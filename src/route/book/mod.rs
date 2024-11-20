mod create_book;
mod create_chapter;
mod delete_book;
mod delete_chapter;
mod get_book_info;
mod get_chapter_info;

use axum::{
    routing::{get, post},
    Router,
};

pub async fn init() -> Router {
    Router::new()
        .route("/", get(book_root).post(post(create_book::create_book)))
        .route(
            "/:book_id",
            get(get_book_info::get_book_info).post(create_chapter::create_chapter),
        )
        .route(
            "/:book_id/:chapter_id",
            get(get_chapter_info::get_chapter_info),
        )
}

async fn book_root() -> &'static str {
    "this is api book root path"
}
