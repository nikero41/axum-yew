pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: String,
    pub origins: Vec<String>,
    pub log_level: String,
    pub db_pool_size: u32,
    pub request_limit: u32,
}

impl Config {
    pub fn load() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: std::env::var("HOST").unwrap_or("127.0.0.1".to_string()),
            port: std::env::var("PORT").unwrap_or("3000".to_string()),
            origins: std::env::var("ORIGINS")
                .expect("ORIGINS must be set")
                .split(',')
                .map(String::from)
                .collect(),
            log_level: std::env::var("LOG_LEVEL").expect("LOG_LEVEL must be set"),
            db_pool_size: std::env::var("DB_POOL_SIZE")
                .expect("DB_POOL_SIZE must be set")
                .parse()
                .expect("DB_POOL_SIZE must be a number"),
            request_limit: std::env::var("REQUEST_LIMIT")
                .expect("REQUEST_LIMIT must be set")
                .parse()
                .expect("REQUEST_LIMIT must be a number"),
        }
    }
}
