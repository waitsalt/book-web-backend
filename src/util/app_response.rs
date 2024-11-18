use serde::{de::DeserializeOwned, Deserialize, Serialize};

use axum::response::IntoResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: DeserializeOwned + Serialize> AppResponse<T> {
    pub fn from(message: String, data: Option<T>) -> Self {
        Self {
            code: 200,
            message: message,
            data: data,
        }
    }

    pub fn success(data: Option<T>) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: data,
        }
    }
}

impl<T: DeserializeOwned + Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
