use crate::route;
use crate::util::config::CONFIG;
use crate::util::{database, logger, redis};
use axum::Router;

pub async fn init() -> Router {
    logger::init().await;
    database::init().await;
    redis::init().await;
    route::init().await
}
