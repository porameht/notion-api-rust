mod domain;
mod application;
mod infrastructure;
mod api;

use dotenv::dotenv;
use std::env;
use infrastructure::notion::NotionClient;
use application::services::NotionService;
use axum::{Router, routing::{get, post}};

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let database_id = env::var("NOTION_DATABASE_ID")
        .expect("NOTION_DATABASE_ID must be set");
    let api_token = env::var("NOTION_API_TOKEN")
        .expect("NOTION_API_TOKEN must be set");
    let daily_spin_limit = env::var("DAILY_SPIN_LIMIT")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<i32>()
        .unwrap_or(1);

    let notion_client = NotionClient::new(database_id, api_token, daily_spin_limit);
    let notion_service = NotionService::new(notion_client);
    
    let app = api::routes::create_router(notion_service);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}