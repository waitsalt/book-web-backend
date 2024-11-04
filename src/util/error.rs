use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub enum BookError {
    NoUploadFile,
    UploadFileFormatError,
    Other,
}

#[derive(Debug, Error)]
#[error("...")]
pub enum AppError {
    BookError(#[from] BookError),
    Other,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            // book
            AppError::BookError(BookError::NoUploadFile) => {
                (StatusCode::BAD_REQUEST, 0000, "no found upload file")
            }
            AppError::BookError(BookError::UploadFileFormatError) => {
                (StatusCode::BAD_REQUEST, 0001, "upload file format wrong")
            }

            // final
            _ => (StatusCode::BAD_REQUEST, 4000, "not known wrong"),
        };
        let body = Json(json!({
            "code":code,
            "message": message,
        }));
        (status_code, body).into_response()
    }
}
