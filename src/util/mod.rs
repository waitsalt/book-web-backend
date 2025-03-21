pub mod app_error;
pub mod app_response;
pub mod auth;
pub mod captcha_email;
pub mod config;
pub mod database;
pub mod email;
pub mod logger;
pub mod redis;

use app_error::AppError;
use app_response::AppResponse;

pub type AppResult<T> = std::result::Result<AppResponse<T>, AppError>;
pub type Result<T> = std::result::Result<T, AppError>;
