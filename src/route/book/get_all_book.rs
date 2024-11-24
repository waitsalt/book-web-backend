use crate::{
    model::{book::Book, user::ClaimsUser},
    sql,
    util::{app_response::AppResponse, auth::check_admin, database::get_pool, AppResult},
};

pub async fn get_all_book(claims_user_opt: Option<ClaimsUser>) -> AppResult<Vec<Book>> {
    let _claims_user = check_admin(claims_user_opt).await?;
    let pool = get_pool().await;
    let books = sql::book::get_all_book(pool).await?;
    Ok(AppResponse::success(Some(books)))
}
