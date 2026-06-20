use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, product::Product};

pub fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_products).post(create_product))
        .route(
            "/{id}",
            get(get_product).put(update_product).delete(delete_product),
        )
}

pub async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    let products = state.product_service.list().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {}", err),
        )
    })?;

    Ok(Json(products))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let id = id
        .parse::<uuid::Uuid>()
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Error is: {}", err)))?;

    let product = state.product_service.find_by_id(id).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {}", err),
        )
    })?;

    if let Some(product) = product {
        Ok(Json(product))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("could not find product with id: {}", id),
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewProductInput {
    name: String,
    price: i32,
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(new_product): Json<NewProductInput>,
) -> Result<(StatusCode, Json<Product>), (StatusCode, String)> {
    let product = state
        .product_service
        .create(new_product.name, new_product.price)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is: {}", err),
            )
        })?;

    Ok((StatusCode::CREATED, Json(product)))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let id = id
        .parse::<uuid::Uuid>()
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Error is: {}", err)))?;

    let product = state.product_service.update(id).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {}", err),
        )
    })?;

    if let Some(product) = product {
        Ok(Json(product))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("could not find product with id: {}", id),
        ))
    }
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let id = id
        .parse::<uuid::Uuid>()
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Error is: {}", err)))?;

    println!("🪚 id: {:?}", id);
    let product = state.product_service.delete(id).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {}", err),
        )
    })?;
    println!("🪚 product: {:?}", product);

    if let Some(product) = product {
        println!("🪚 💜");
        Ok(Json(product))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("could not find product with id: {}", id),
        ))
    }
}
