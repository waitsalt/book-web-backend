use crate::{
    model::book::BookInfo,
    util::{app_response::AppResponse, AppResult},
};

pub async fn latest_update_books() -> AppResult<Vec<BookInfo>> {
    Ok(AppResponse::success(None))
}
