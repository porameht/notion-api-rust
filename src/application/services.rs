use crate::domain::{
    models::{SpinResult, GameType},
    repository::{NotionRepository, Error},
};

#[derive(Clone)]
pub struct NotionService<R: NotionRepository + Clone> {
    repository: R,
}

impl<R: NotionRepository + Clone> NotionService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_spin_result(&self, spin_result: SpinResult, game_type: GameType) -> Result<(), Error> {
        self.repository.create_entry(spin_result, game_type).await
    }

    pub async fn get_spin_results(&self, game_type: GameType) -> Result<Vec<SpinResult>, Error> {
        self.repository.get_entries(game_type).await
    }

    pub async fn update_spin_result(&self, page_id: &str, spin_result: SpinResult, game_type: GameType) -> Result<(), Error> {
        self.repository.update_entry(page_id, spin_result, game_type).await
    }

    pub async fn delete_spin_result(&self, page_id: &str, game_type: GameType) -> Result<(), Error> {
        self.repository.delete_entry(page_id, game_type).await
    }
} 