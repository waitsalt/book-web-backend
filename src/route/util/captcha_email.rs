// use axum::extract::Path;
// use lettre::{
//     message::header::ContentType, transport::smtp::authentication::Credentials, Message,
//     SmtpTransport, Transport,
// };

// use crate::util::{
//     app_error::AppError, app_response::AppResponse, config::CONFIG, redis::get_redis_connect,
//     AppResult,
// };

// pub async fn captcha_email(Path(user_email): Path<String>) -> AppResult<()> {
//     let email_send = CONFIG.email.username.clone();
//     let password = CONFIG.email.password.clone();
//     let host = CONFIG.email.host.clone();
//     let port = CONFIG.email.port;

//     tracing::info!("config load");

//     let verification_code_key = format!("verification_code_key:{}", user_email);
//     let verification_code = nanoid::nanoid!(6);

//     tracing::info!("vode load");

//     let mut con = get_redis_connect().await;
//     let _: () = redis::cmd("SET")
//         .arg(verification_code_key)
//         .arg(verification_code.clone())
//         .arg("EX")
//         .arg(5 * 60)
//         .query(&mut con)
//         .unwrap();

//     tracing::info!("start send message");

//     let message = Message::builder()
//         .from(email_send.parse().unwrap())
//         .to(user_email.parse().unwrap())
//         .subject("验证码")
//         .header(ContentType::TEXT_PLAIN)
//         .body("你的验证码是 ".to_string() + verification_code.as_str())
//         .unwrap();
//     let creds = Credentials::new(email_send.to_owned(), password.to_owned());

//     let mailer = SmtpTransport::starttls_relay(&host)
//         .unwrap()
//         .port(port)
//         .credentials(creds)
//         // .authentication(mechanisms)
//         .build();
//     mailer.send(&message).unwrap();
//     Ok(AppResponse::success(None))
//     // match mailer.send(&message) {
//     //     Ok(_) => return Ok(AppResponse::success(None)),
//     //     Err(_) => return Err(AppError::EmailSendFail),
//     // }
// }

use axum::extract::Path;

use crate::util::{app_response::AppResponse, AppResult};

pub async fn captcha_email(Path(user_email): Path<String>) -> AppResult<String> {
    Ok(AppResponse::success(Some(user_email)))
}
