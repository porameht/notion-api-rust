use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use chrono::Utc;
use tracing::{info, warn, error, debug};
use std::collections::HashMap;

use crate::domain::{
    models::*,
    repository::{NotionRepository, Error},
};

#[derive(Clone)]
pub struct NotionClient {
    client: Client,
    database_ids: HashMap<GameType, String>,
    api_token: String,
    daily_spin_limit: i32,
}

impl NotionClient {
    pub fn new(database_ids: HashMap<GameType, String>, api_token: String, daily_spin_limit: i32) -> Self {
        let client = Client::new();
        Self {
            client,
            database_ids,
            api_token,
            daily_spin_limit,
        }
    }

    fn get_database_id(&self, game_type: GameType) -> Result<&String, Error> {
        self.database_ids.get(&game_type)
            .ok_or_else(|| Error::NotionApi(format!("No database ID configured for game type: {:?}", game_type)))
    }

    fn build_properties(&self, spin_result: &SpinResult) -> NotionProperties {
        // Convert game_type to string if present
        let _game_type_str = spin_result.game_type.clone().unwrap_or_default();
        
        NotionProperties {
            key: NotionTitle {
                r#type: "title".to_string(),
                title: vec![NotionText {
                    r#type: "text".to_string(),
                    text: NotionTextContent {
                        content: spin_result.key.clone(),
                    },
                }],
            },
            datetime: NotionDate {
                r#type: "date".to_string(),
                date: NotionDateContent {
                    start: spin_result.datetime.clone(),
                },
            },
            number: NotionNumber {
                r#type: "number".to_string(),
                number: spin_result.number,
            },
            is_win: NotionCheckbox {
                r#type: "checkbox".to_string(),
                checkbox: spin_result.is_win,
            },
            checked: NotionCheckbox {
                r#type: "checkbox".to_string(),
                checkbox: spin_result.checked,
            },
        }
    }

    async fn has_reached_spin_limit(&self, key: &str, game_type: GameType) -> Result<bool, Error> {
        debug!("Checking spin limit for key: {} with game type: {:?}", key, game_type);
        let today = Utc::now().date_naive();
        let database_id = self.get_database_id(game_type)?;
        
        let response = self.client
            .post(format!("https://api.notion.com/v1/databases/{}/query", database_id))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "filter": {
                    "and": [
                        {
                            "property": "key",
                            "title": {
                                "equals": key
                            }
                        },
                        {
                            "property": "datetime",
                            "date": {
                                "after": format!("{}T00:00:00Z", today),
                                "before": format!("{}T23:59:59Z", today)
                            }
                        }
                    ]
                }
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::NotionApi(response.text().await?));
        }

        let data: serde_json::Value = response.json().await?;
        let results = data["results"].as_array()
            .ok_or_else(|| Error::NotionApi("Invalid response format".to_string()))?;

        let count = results.len();
        debug!("Found {} spins today for key: {} with game type: {:?}", count, key, game_type);
        Ok(count >= self.daily_spin_limit as usize)
    }
}

#[async_trait]
impl NotionRepository for NotionClient {
    async fn create_entry(&self, spin_result: SpinResult, game_type: GameType) -> Result<(), Error> {
        info!(
            "Creating new result for key {} with number {} for game type: {:?}",
            spin_result.key, spin_result.number, game_type
        );

        if self.has_reached_spin_limit(&spin_result.key, game_type).await? {
            warn!(
                "Daily limit reached for key: {} with game type: {:?}",
                spin_result.key, game_type
            );
            return Err(Error::SpinLimitReached);
        }

        let database_id = self.get_database_id(game_type)?;
        let properties = self.build_properties(&spin_result);
        
        let response = self.client
            .post("https://api.notion.com/v1/pages")
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "parent": { "database_id": database_id },
                "properties": properties
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Notion API error: {}", error_text);
            return Err(Error::NotionApi(error_text));
        }

        info!("Successfully created result for game type: {:?}", game_type);
        Ok(())
    }

    async fn get_entries(&self, game_type: GameType) -> Result<Vec<SpinResult>, Error> {
        debug!("Fetching all results for game type: {:?}", game_type);
        let database_id = self.get_database_id(game_type)?;
        
        let response = self.client
            .post(format!("https://api.notion.com/v1/databases/{}/query", database_id))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "page_size": 100
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::NotionApi(response.text().await?));
        }

        let data: serde_json::Value = response.json().await?;
        let results = data["results"].as_array()
            .ok_or_else(|| Error::NotionApi("Invalid response format".to_string()))?;

        let mut spin_results = Vec::new();
        for result in results {
            let properties = &result["properties"];
            
            let key = properties["key"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();
            
            let datetime = properties["datetime"]["date"]["start"]
                .as_str()
                .unwrap_or("")
                .to_string();
            
            let number = properties["number"]["number"]
                .as_i64()
                .unwrap_or(0) as i32;
            
            let is_win = properties["isWin"]["checkbox"]
                .as_bool()
                .unwrap_or(false);
                
            let checked = properties["checked"]["checkbox"]
                .as_bool()
                .unwrap_or(false);

            spin_results.push(SpinResult {
                key,
                datetime,
                number,
                is_win,
                checked,
                game_type: Some(format!("{:?}", game_type)),
            });
        }

        info!("Successfully fetched {} results for game type: {:?}", spin_results.len(), game_type);
        Ok(spin_results)
    }

    async fn update_entry(&self, page_id: &str, spin_result: SpinResult, game_type: GameType) -> Result<(), Error> {
        info!(
            "Updating result {} for key {} with number {} for game type: {:?}",
            page_id, spin_result.key, spin_result.number, game_type
        );
        
        let properties = self.build_properties(&spin_result);
        
        let response = self.client
            .patch(format!("https://api.notion.com/v1/pages/{}", page_id))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "properties": properties
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to update result {}: {}", page_id, error_text);
            return Err(Error::NotionApi(error_text));
        }

        info!("Successfully updated result {} for game type: {:?}", page_id, game_type);
        Ok(())
    }

    async fn delete_entry(&self, page_id: &str, game_type: GameType) -> Result<(), Error> {
        info!("Deleting result {} for game type: {:?}", page_id, game_type);
        
        let response = self.client
            .patch(format!("https://api.notion.com/v1/pages/{}", page_id))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "archived": true
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to delete result {}: {}", page_id, error_text);
            return Err(Error::NotionApi(error_text));
        }

        info!("Successfully deleted result {} for game type: {:?}", page_id, game_type);
        Ok(())
    }
} 