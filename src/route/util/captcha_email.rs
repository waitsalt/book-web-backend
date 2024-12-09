use axum::extract::Path;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::util::{
    app_error::AppError, app_response::AppResponse, config::CONFIG, redis::get_redis_connect,
    AppResult,
};

pub async fn captcha_email(Path(user_email): Path<String>) -> AppResult<()> {
    let email_send = CONFIG.email.username.clone();
    let password = CONFIG.email.password.clone();
    let host = CONFIG.email.host.clone();
    let port = CONFIG.email.port;

    let captcha_email_key = format!("captcha_email_key:{}", user_email);
    let verify_code = nanoid::nanoid!(6);

    let mut con = get_redis_connect().await;
    let _: () = redis::cmd("SET")
        .arg(captcha_email_key)
        .arg(verify_code.clone())
        .arg("EX")
        .arg(5 * 60)
        .query(&mut con)
        .unwrap();

    let message = Message::builder()
        .from(email_send.parse().unwrap())
        .to(user_email.parse().unwrap())
        .subject("验证码")
        .header(ContentType::TEXT_PLAIN)
        .body("你的验证码是 ".to_string() + verify_code.as_str())
        .unwrap();
    let creds = Credentials::new(email_send.to_owned(), password.to_owned());

    let mailer = SmtpTransport::starttls_relay(&host)
        .unwrap()
        .port(port)
        .credentials(creds)
        .build();
    match mailer.send(&message) {
        Ok(_) => return Ok(AppResponse::success(None)),
        Err(_) => return Err(AppError::EmailSendFail),
    }
}
