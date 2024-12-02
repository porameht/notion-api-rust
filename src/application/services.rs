use crate::domain::{
    models::SpinResult,
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

    pub async fn create_spin_result(&self, spin_result: SpinResult) -> Result<(), Error> {
        self.repository.create_entry(spin_result).await
    }

    pub async fn get_spin_results(&self) -> Result<Vec<SpinResult>, Error> {
        self.repository.get_entries().await
    }

    pub async fn update_spin_result(&self, page_id: &str, spin_result: SpinResult) -> Result<(), Error> {
        self.repository.update_entry(page_id, spin_result).await
    }

    pub async fn delete_spin_result(&self, page_id: &str) -> Result<(), Error> {
        self.repository.delete_entry(page_id).await
    }
} 