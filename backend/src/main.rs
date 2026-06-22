use anyhow::Result;
use backend::{
    app::{AppState, start_server},
    config::Config,
    db::init_db,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = Config::load()?;

    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .pretty()
        .init();

    let db = init_db(&config.database_url, config.db_pool_size).await?;
    let app_state = AppState::new(db);

    let listener = tokio::net::TcpListener::bind(&config.address).await?;
    tracing::info!("Listening on {}", config.address);
    start_server(listener, app_state, &config).await?;

    Ok(())
}
