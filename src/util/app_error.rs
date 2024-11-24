use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // user
    UserNameExist,
    UserEmailExist,
    UserPasswordShort,
    UserNotExist,
    UserBlocked,
    UserDeleted,
    UserMissPermission,
    UserCreateFailure,
    UserPasswordError,

    // author
    AuthorExist,
    AuthorNotExist,

    // book
    BookExist,
    BookNotExist,

    // chapter
    ChapterExist,
    ChapterNotExist,

    // email
    EmailSendFail,

    // auth
    TokenInvalid,
    TokenMiss,
    CaptchaImageError,
    CaptchaEmailError,

    // sqlx
    SqlxError,

    // notfount
    NotFound,

    // other
    Other,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<sqlx::error::Error> for AppError {
    fn from(_: sqlx::error::Error) -> Self {
        AppError::SqlxError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let bad_request = StatusCode::BAD_REQUEST;
        let (status_code, code, message) = match self {
            // user
            AppError::UserNameExist => (bad_request, 1001, "user name exist"),
            AppError::UserEmailExist => (bad_request, 1002, "user email exist"),
            AppError::UserPasswordShort => (bad_request, 1002, "user password too short"),
            AppError::UserNotExist => (bad_request, 1003, "用户不存在"),
            AppError::UserBlocked => (bad_request, 1004, "用户被封禁"),
            AppError::UserDeleted => (bad_request, 1005, "用户已被删除"),
            AppError::UserMissPermission => (bad_request, 1006, "用户没有权限"),
            AppError::UserCreateFailure => (bad_request, 1007, "用户创建失败"),
            AppError::UserPasswordError => (bad_request, 1008, "用户密码错误"),

            // author
            AppError::AuthorExist => (bad_request, 1101, "作者已存在"),
            AppError::AuthorNotExist => (bad_request, 1101, "作者不存在"),

            // book
            AppError::BookExist => (bad_request, 1201, "书籍已存在"),
            AppError::BookNotExist => (bad_request, 1202, "书籍不存在"),

            // chapter
            AppError::ChapterExist => (bad_request, 1301, "章节已存在"),
            AppError::ChapterNotExist => (bad_request, 1302, "章节不存在"),

            // email
            AppError::EmailSendFail => (bad_request, 1401, "邮件未发送成功"),

            // auth
            AppError::TokenInvalid => (bad_request, 2001, "无效的 token"),
            AppError::TokenMiss => (bad_request, 2002, "用户未登陆"),
            AppError::CaptchaEmailError => (bad_request, 2003, "邮箱验证码错误"),
            AppError::CaptchaImageError => (bad_request, 2004, "图形验证码错误"),

            // sqlx
            AppError::SqlxError => (bad_request, 3001, "sql 出现问题"),

            // not found
            AppError::NotFound => (bad_request, 4004, "not found"),

            // Other
            AppError::Other => (bad_request, 5000, "not known wrong"),
        };
        let body = Json(json!({
            "code": code,
            "message": message,
        }));
        (status_code, body).into_response()
    }
}
