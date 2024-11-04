use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub enum BookError {}

#[derive(Debug, Error)]
#[error("...")]
pub enum AppError {
    BookError(#[from] BookError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            // book

            // final
            _ => (StatusCode::BAD_REQUEST, 4000, "wrong request"),
        };
        let body = Json(json!({
            "code":code,
            "message": message,
        }));
        (status_code, body).into_response()
    }
}
