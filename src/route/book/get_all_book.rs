use crate::{
    model::{book::Book, user::UserClaims},
    sql,
    util::{app_response::AppResponse, auth::check_admin, database::get_pool, AppResult},
};

pub async fn get_all_book(claims_user_opt: Option<UserClaims>) -> AppResult<Vec<Book>> {
    let _claims_user = check_admin(claims_user_opt).await?;
    let pool = get_pool().await;
    let books = sql::book::get_all_book(pool).await?;
    Ok(AppResponse::success(Some(books)))
}
