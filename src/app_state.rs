use crate::product::ProductService;

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
}

impl AppState {
    pub fn new(db: crate::db::DbPool) -> Self {
        Self {
            product_service: ProductService::new(db),
        }
    }
}
