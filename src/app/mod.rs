use crate::component::router;
use crate::util::{database, logger};
use axum::Router;

pub async fn init() -> Router {
    logger::init().await;
    database::init().await;
    router::init().await
}
