use once_cell::sync::OnceCell;

use super::config::CONFIG;

static CLIENT: OnceCell<redis::Client> = OnceCell::new();

pub async fn init() {
    let redis_url = CONFIG.redis.url.clone();

    let client = redis::Client::open(redis_url).expect("redis open error");
    assert!(CLIENT.set(client).is_ok(), "redis init error");
}

pub async fn get_redis_connect() -> redis::Connection {
    CLIENT
        .get()
        .expect("redis client get error")
        .get_connection()
        .expect("redis connect get error")
}
