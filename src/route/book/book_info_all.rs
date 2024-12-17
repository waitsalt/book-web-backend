use crate::{
    model::{book::BookInfo, user::UserClaims},
    sql,
    util::{app_response::AppResponse, auth::check_admin, database::get_pool, AppResult},
};

pub async fn book_info_all(claims_user_opt: Option<UserClaims>) -> AppResult<Vec<BookInfo>> {
    let _claims_user = check_admin(claims_user_opt).await?;
    let pool = get_pool().await;
    let books = sql::book::get_all_book(pool).await?;
    Ok(AppResponse::success(Some(books)))
}
