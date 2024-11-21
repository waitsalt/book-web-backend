mod create_book;
mod create_chapter;
mod delete_book;
mod delete_chapter;
mod get_all_book;
mod get_all_chapter;
mod get_book_info;
mod get_chapter_info;
mod search_book;

use axum::{
    routing::{get, post},
    Router,
};

pub async fn init() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_book::get_all_book).post(create_book::create_book),
        )
        .route("/search", post(search_book::search_book))
        .route(
            "/:book_id",
            get(get_book_info::get_book_info).post(create_chapter::create_chapter),
        )
        .route("/:book_id/chapters", get(get_all_chapter::get_all_chapter))
        .route(
            "/:book_id/:chapter_id",
            get(get_chapter_info::get_chapter_info),
        )
}

// async fn book_root() -> &'static str {
//     "this is api book root path"
// }
