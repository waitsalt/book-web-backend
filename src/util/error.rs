use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub enum BookError {
    NoUploadFile,
    UploadFileFormatError,
    NotUploadSql,
    BookExist,
}

#[derive(Debug, Error)]
#[error("...")]
pub enum DatabaseError {
    PoolGetError,
    SqlRunError,
}

#[derive(Debug, Error)]
#[error("...")]
pub enum AppError {
    BookError(#[from] BookError),
    DatabaseError(#[from] DatabaseError),
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
            AppError::BookError(BookError::NotUploadSql) => {
                (StatusCode::BAD_REQUEST, 0002, "can not write in sql")
            }
            AppError::BookError(BookError::BookExist) => {
                (StatusCode::BAD_REQUEST, 0003, "book is exit")
            }

            // database
            AppError::DatabaseError(DatabaseError::PoolGetError) => {
                (StatusCode::BAD_REQUEST, 1000, "connect sql pool failure")
            }
            AppError::DatabaseError(DatabaseError::SqlRunError) => {
                (StatusCode::BAD_REQUEST, 1001, "sql run error")
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
