use std::{sync::Arc, time::Duration};
use tokio::signal;

use crate::{config::Config, handlers::router, product::ProductService};
use axum::http::{Method, StatusCode};
use tower_http::{
    LatencyUnit,
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: crate::db::DbPool, config: &Config) -> Self {
        Self {
            product_service: ProductService::new(db),
            config: Arc::new(config.clone()),
        }
    }
}

pub async fn start_server(listener: tokio::net::TcpListener, state: AppState) {
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

    let origins = state
        .config
        .origins
        .iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(origins);

    let app = router()
        .layer(cors_layer)
        .layer(trace_layer)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .with_state(state);

    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
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
