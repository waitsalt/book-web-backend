pub mod app_error;
pub mod app_response;
pub mod auth;
pub mod config;
pub mod database;
pub mod logger;
pub mod redis;

use app_error::AppError;
use app_response::AppResponse;

pub type AppResult<T> = std::result::Result<AppResponse<T>, AppError>;
