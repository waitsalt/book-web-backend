use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::init().expect("config init failure"));

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub secret: String,
    pub refresh_token_duration: i64,
    pub access_token_duration: i64,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub auth: Auth,
    pub server: Server,
    pub database: Database,
    pub redis: Redis,
    pub logger: Logger,
    pub data: Data,
    pub email: Email,
}

impl Config {
    pub fn init() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name("config/user"))
            .add_source(config::Environment::default().separator("_"));
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}
