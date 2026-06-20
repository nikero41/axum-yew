use std::time::Duration;
use tokio::signal;

use anyhow::Result;
use axum::{
    Router,
    http::{HeaderValue, Method, StatusCode},
};
use full_stack_crud::{app_state::AppState, db::init_db, handlers::router};
use tower_http::{
    LatencyUnit,
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = Config::load();

    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .pretty()
        .init();

    let db = init_db(&config.database_url, config.db_pool_size).await?;
    let app = app().with_state(AppState::new(db));

    let listener = tokio::net::TcpListener::bind(&config.address)
        .await
        .unwrap();
    tracing::info!("Listening on {}", config.address);
    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;

    Ok(())
}

fn app() -> Router<AppState> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_failure(
            DefaultOnFailure::new()
                .level(Level::ERROR)
                .latency_unit(LatencyUnit::Micros),
        );
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());

    router()
        .layer(cors_layer)
        .layer(trace_layer)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
