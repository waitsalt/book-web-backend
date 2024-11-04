use crate::util::config::CONFIG;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tokio::sync::OnceCell;

pub static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn init() {
    POOL.get_or_init(|| async {
        let database_url = CONFIG.database.url.as_str();
        PgPoolOptions::new()
            .max_connections(6)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("Failed to connect to the database")
    })
    .await;
}
