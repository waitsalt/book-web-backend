use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};
use tower_http::trace;

pub mod book;

pub async fn init() -> Router {
    let book_router = book::init().await;

    Router::new()
        .nest("/book", book_router)
        .fallback(fallback)
        .layer(
            trace::TraceLayer::new_for_http(), //.make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                                               //.on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                                               //.on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
        )
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
