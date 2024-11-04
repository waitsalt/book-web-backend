mod app;
mod component;
mod util;

use util::config::CONFIG;

#[tokio::main]
async fn main() {
    let app = app::init().await;
    let address = format!("127.0.0.1:{}", CONFIG.server.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("address bind failure");
    tracing::info!("server run in http://{address}");
    axum::serve(listener, app).await.expect("app run failure")
}
