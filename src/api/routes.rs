use axum::{
    Router,
    routing::{post, get, put, delete},
};
use tower_http::cors::{CorsLayer, Any};
use crate::application::services::NotionService;
use crate::infrastructure::notion::NotionClient;

pub fn create_router(service: NotionService<NotionClient>) -> Router {
    // Add CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/spin-results", post(super::handlers::create_spin_result))
        .route("/spin-results", get(super::handlers::get_spin_results))
        .route("/spin-results/:page_id", put(super::handlers::update_spin_result))
        .route("/spin-results/:page_id", delete(super::handlers::delete_spin_result))
        .layer(cors)
        .with_state(service)
} 