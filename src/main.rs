mod app;
mod component;
mod util;

use crate::app::run;

#[tokio::main]
async fn main() {
    run().await;
}
