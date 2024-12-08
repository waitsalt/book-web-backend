mod create_book;
mod create_chapter;
mod delete_book;
mod delete_chapter;
mod get_all_book;
mod get_book_info;
mod get_chapter;
mod get_chapter_list;
mod latest_update_books;
mod search_book;

use axum::{
    routing::{get, post},
    Router,
};
use latest_update_books::latest_update_books;

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
        .route(
            "/:book_id/chapter_list",
            get(get_chapter_list::get_chapter_list),
        )
        .route("/:book_id/:chapter_id", get(get_chapter::get_chapter))
        .route("/latest_update", get(latest_update_books))
}

// async fn book_root() -> &'static str {
//     "this is api book root path"
// }
