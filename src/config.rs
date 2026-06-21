use std::{env, net::SocketAddr};

use garde::Validate;
use tracing::Level;

#[derive(Debug, Clone, Validate)]
pub struct Config {
    #[garde(url)]
    pub database_url: String,
    #[garde(skip)]
    pub address: SocketAddr,
    #[garde(length(min = 1))]
    pub origins: Vec<HeaderValue>,
    #[garde(skip)]
    pub log_level: Level,
    #[garde(range(min = 1))]
    pub db_pool_size: u32,
    #[garde(skip)]
    pub request_limit: Option<u32>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let log_level = match env::var("LOG_LEVEL").as_deref() {
            Ok("debug") => Level::DEBUG,
            Ok("info") => Level::INFO,
            Ok("warn") => Level::WARN,
            Ok("error") => Level::ERROR,
            Ok(_) => {
                panic!("LOG_LEVEL must be one of debug, info, warn, or error")
            }
            Err(_) => Level::INFO,
        };

        let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or("3001".to_string())
            .parse::<u16>()
            .expect("Invalid port number");

        let config = Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            address: SocketAddr::new(host.parse().expect("Invalid host"), port),
            origins: env::var("ORIGINS")
                .unwrap_or("http://localhost:3000".to_string())
                .split(',')
                .map(String::from)
                .collect(),
            log_level,
            db_pool_size: env::var("DB_POOL_SIZE")
                .unwrap_or("5".to_string())
                .parse()
                .context("DB_POOL_SIZE must be a number")?,
            request_limit: env::var("REQUEST_LIMIT")
                .ok()
                .map(|s| s.parse().expect("REQUEST_LIMIT must be a number")),
        }
        config.validate()?;

        Ok(config)
    }
}
