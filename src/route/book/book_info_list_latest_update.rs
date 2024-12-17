use crate::{
    model::book::BookInfo,
    sql,
    util::{app_response::AppResponse, database::get_pool, AppResult},
};

pub async fn book_info_list_latest_update() -> AppResult<Vec<BookInfo>> {
    let pool = get_pool().await;
    let book_info_list = sql::book::book_list_latest_update(pool).await.unwrap();
    Ok(AppResponse::success(Some(book_info_list)))
}
