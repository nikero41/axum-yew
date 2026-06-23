use std::{env, net::SocketAddr};

use anyhow::{Context, Result};
use axum::http::HeaderValue;
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
    #[garde(skip)]
    pub request_timeout_seconds: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let log_level = match env::var("LOG_LEVEL") {
            Ok(debug) if debug.eq_ignore_ascii_case("debug") => Level::DEBUG,
            Ok(info) if info.eq_ignore_ascii_case( "info") => Level::INFO,
            Ok(warn) if warn.eq_ignore_ascii_case("warn") => Level::WARN,
            Ok(error) if error.eq_ignore_ascii_case( "error") => Level::ERROR,
            Ok(_) => anyhow::bail!("LOG_LEVEL must be one of debug, info, warn, or error"),
            Err(_) => Level::INFO,
        };

        let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or("3001".to_string())
            .parse::<u16>()
            .context("Invalid port number")?;

        let config = Self {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            address: SocketAddr::new(host.parse().context("Invalid host")?, port),
            origins: env::var("ORIGINS")
                .unwrap_or("http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.parse().context("ORIGINS must be a valid url"))
                .collect::<Result<Vec<HeaderValue>>>()?,
            log_level,
            db_pool_size: env::var("DB_POOL_SIZE")
                .unwrap_or("5".to_string())
                .parse()
                .context("DB_POOL_SIZE must be a number")?,
            request_limit: env::var("REQUEST_LIMIT")
                .ok()
                .map(|s| s.parse().context("REQUEST_LIMIT must be a number"))
                .transpose()?,
            request_timeout_seconds: env::var("REQUEST_TIMEOUT_SECONDS")
                .unwrap_or("10".to_string())
                .parse()
                .context("REQUEST_LIMIT must be a number")?,
        };

        config.validate()?;

        Ok(config)
    }
}
