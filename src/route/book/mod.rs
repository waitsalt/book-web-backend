mod book_create;
mod book_info;
mod book_info_all;
mod book_info_list_latest_update;
mod book_search;
mod book_verify;
mod chapter;
mod chapter_create;
mod chapter_info_list;

use axum::{
    routing::{get, post},
    Router,
};

pub async fn init() -> Router {
    Router::new()
        .route(
            "/",
            get(book_info_all::book_info_all).post(book_create::book_create),
        )
        .route("/search", post(book_search::book_search))
        .route(
            "/:book_id",
            get(book_info::book_info).post(chapter_create::chapter_create),
        )
        .route(
            "/:book_id/chapter_list",
            get(chapter_info_list::chapter_info_list),
        )
        .route("/:book_id/:chapter_id", get(chapter::chapter))
        .route(
            "/latest_update",
            get(book_info_list_latest_update::book_info_list_latest_update),
        )
        .route("/verify_book", post(book_verify::book_verify))
}

// async fn book_root() -> &'static str {
//     "this is api book root path"
// }
