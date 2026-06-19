use axum::{Router, http::StatusCode, routing::get};

use crate::{app_state::AppState, handlers::product::product_routes};

mod product;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/products", product_routes())
        .fallback(fallback)
}

pub async fn health_check() -> &'static str {
    "ok"
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
