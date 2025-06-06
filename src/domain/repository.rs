use async_trait::async_trait;
use crate::domain::models::{SpinResult, GameType};

#[async_trait]
pub trait NotionRepository {
    async fn create_entry(&self, spin_result: SpinResult, game_type: GameType) -> Result<(), Error>;
    async fn get_entries(&self, game_type: GameType) -> Result<Vec<SpinResult>, Error>;
    async fn update_entry(&self, page_id: &str, spin_result: SpinResult, game_type: GameType) -> Result<(), Error>;
    async fn delete_entry(&self, page_id: &str, game_type: GameType) -> Result<(), Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Daily spin limit reached")]
    SpinLimitReached,
    #[error("Notion API error: {0}")]
    NotionApi(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
} 