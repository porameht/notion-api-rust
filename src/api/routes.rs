use axum::{
    Router,
    routing::{post, get, put, delete},
};
use tower_http::cors::{CorsLayer, Any};
use crate::application::services::NotionService;
use crate::infrastructure::notion::NotionClient;
use std::env;
use http::{HeaderValue, Method};

pub fn create_router(service: NotionService<NotionClient>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]);
    
    let cors = match env::var("ALLOWED_ORIGINS") {
        Ok(origins) => {
            let origins: Vec<HeaderValue> = origins
                .split(',')
                .filter_map(|origin| origin.trim().parse().ok())
                .collect();
            
            if origins.is_empty() {
                cors.allow_origin(Any)
            } else {
                origins.iter().fold(cors, |cors, origin| {
                    cors.allow_origin(origin.clone())
                })
            }
        },
        Err(_) => {
            cors.allow_origin(Any)
        }
    }
    .allow_headers(Any);

    Router::new()
        .route("/", get(super::handlers::get_root))
        .route("/spin-results", post(super::handlers::create_spin_result))
        .route("/spin-results", get(super::handlers::get_spin_results))
        .route("/spin-results/:page_id", put(super::handlers::update_spin_result))
        .route("/spin-results/:page_id", delete(super::handlers::delete_spin_result))
        .route("/spin-result", post(|state, json| async move {
            super::handlers::spin_result(state, json).await
        }))
        .route("/wheel-result", post(|state, json| async move {
            super::handlers::wheel_result(state, json).await
        }))
        .layer(cors)
        .with_state(service)
} 