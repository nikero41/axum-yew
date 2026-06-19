use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::query_as;

use crate::db::DbPool;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: uuid::Uuid,
    pub name: String,
    pub price: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct ProductService {
    db: DbPool,
}

impl ProductService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, name: String, price: i32) -> Result<Product> {
        query_as!(
            Product,
            "INSERT INTO products (name, price)
                VALUES ($1, $2)
                RETURNING *",
            name,
            price
        )
        .fetch_one(&self.db)
        .await
        .context("failed to create product")
    }

    pub async fn list(&self) -> Result<Vec<Product>> {
        query_as!(Product, "SELECT * FROM products")
            .fetch_all(&self.db)
            .await
            .context("failed to list products")
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Product>> {
        query_as!(Product, "SELECT * FROM products WHERE id=$1", id,)
            .fetch_optional(&self.db)
            .await
            .context("failed to get product")
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<Option<Product>> {
        query_as!(Product, "SELECT * FROM products WHERE id=$1", id,)
            .fetch_optional(&self.db)
            .await
            .context("failed to get product")
    }

    pub async fn update(&self, id: uuid::Uuid) -> Result<Option<Product>> {
        query_as!(Product, "SELECT * FROM products WHERE id=$1", id,)
            .fetch_optional(&self.db)
            .await
            .context("failed to get product")
    }
}
