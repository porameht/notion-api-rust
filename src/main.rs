mod domain;
mod application;
mod infrastructure;
mod api;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use infrastructure::notion::NotionClient;
use application::services::NotionService;
use tracing::{info, Level};
use tracing_subscriber::{FmtSubscriber, EnvFilter};
use crate::domain::models::GameType;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive(Level::INFO.into())
            .add_directive("notion_crud=debug".parse().unwrap()))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .pretty()
        .init();

    info!("Starting Notion CRUD API server");
    
    // Read database IDs for each game type
    let mut database_ids = HashMap::new();
    
    // Check for the legacy database ID first (for backward compatibility)
    if let Ok(legacy_db_id) = env::var("NOTION_DATABASE_ID") {
        info!("Found legacy database ID, using for all game types");
        database_ids.insert(GameType::Spin, legacy_db_id.clone());
        database_ids.insert(GameType::Wheel, legacy_db_id);
    } else {
        // Read specific database IDs for each game
        if let Ok(spin_db_id) = env::var("NOTION_DATABASE_ID_SPIN") {
            info!("Using dedicated database for spin game");
            database_ids.insert(GameType::Spin, spin_db_id);
        } else {
            panic!("NOTION_DATABASE_ID_SPIN must be set if NOTION_DATABASE_ID is not defined");
        }
        
        if let Ok(wheel_db_id) = env::var("NOTION_DATABASE_ID_WHEEL") {
            info!("Using dedicated database for wheel game");
            database_ids.insert(GameType::Wheel, wheel_db_id);
        } else {
            panic!("NOTION_DATABASE_ID_WHEEL must be set if NOTION_DATABASE_ID is not defined");
        }
        
        // Additional game types can be added here in the future
    }
    
    let api_token = env::var("NOTION_API_TOKEN")
        .expect("NOTION_API_TOKEN must be set");
    let daily_spin_limit = env::var("DAILY_SPIN_LIMIT")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<i32>()
        .unwrap_or(1);

    let notion_client = NotionClient::new(database_ids, api_token, daily_spin_limit);
    let notion_service = NotionService::new(notion_client);
    
    let app = api::routes::create_router(notion_service);

    // run our app with hyper
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let addr = format!("0.0.0.0:{}", port);
    info!("Server will listen on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}