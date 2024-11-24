// use lettre::{
//     message::header::ContentType, transport::smtp::authentication::Credentials, Message,
//     SmtpTransport, Transport,
// };

// use crate::util::Result;

// use super::{app_error::AppError, config::CONFIG, redis::get_redis_connect};

// pub async fn send_verification_code(email_receive: &str) -> Result<()> {
//     let email_send = CONFIG.email.username.clone();
//     let password = CONFIG.email.password.clone();
//     let host = CONFIG.email.host.clone();
//     let port = CONFIG.email.port;

//     let verification_code_key = format!("verification_code_key:{}", email_receive);
//     let verification_code = nanoid::nanoid!(6);

//     let mut con = get_redis_connect().await;
//     let _: () = redis::cmd("SET")
//         .arg(verification_code_key)
//         .arg("EX")
//         .arg(verification_code.clone())
//         .arg(5 * 60)
//         .query(&mut con)
//         .unwrap();

//     let message = Message::builder()
//         .from(email_send.parse().unwrap())
//         .to(email_receive.parse().unwrap())
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
//     match mailer.send(&message) {
//         Ok(_) => return Ok(()),
//         Err(_) => return Err(AppError::EmailSendFail),
//     }
// }
