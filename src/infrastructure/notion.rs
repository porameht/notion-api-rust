use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use chrono::{Utc, DateTime};
use tracing::{info, warn, error, debug};

use crate::domain::{
    models::*,
    repository::{NotionRepository, Error},
};

#[derive(Clone)]
pub struct NotionClient {
    client: Client,
    database_id: String,
    api_token: String,
    daily_spin_limit: i32,
}

impl NotionClient {
    pub fn new(database_id: String, api_token: String, daily_spin_limit: i32) -> Self {
        let client = Client::new();
        Self {
            client,
            database_id,
            api_token,
            daily_spin_limit,
        }
    }

    fn build_properties(&self, spin_result: &SpinResult) -> NotionProperties {
        NotionProperties {
            Name: NotionTitle {
                r#type: "title".to_string(),
                title: vec![NotionText {
                    r#type: "text".to_string(),
                    text: NotionTextContent {
                        content: spin_result.name.clone(),
                    },
                }],
            },
            Phone: NotionPhoneNumber {
                r#type: "phone_number".to_string(),
                phone_number: spin_result.phone_number.clone(),
            },
            Ticket: NotionNumber {
                r#type: "number".to_string(),
                number: spin_result.ticket,
            },
            Reward: NotionRichText {
                r#type: "rich_text".to_string(),
                rich_text: vec![NotionText {
                    r#type: "text".to_string(),
                    text: NotionTextContent {
                        content: spin_result.reward.clone(),
                    },
                }],
            },
        }
    }

    async fn has_reached_spin_limit(&self, phone_number: &str) -> Result<bool, Error> {
        debug!("Checking spin limit for phone number: {}", phone_number);
        let today = Utc::now().date_naive();
        
        let response = self.client
            .post(format!("https://api.notion.com/v1/databases/{}/query", self.database_id))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "filter": {
                    "and": [
                        {
                            "property": "Phone",
                            "phone_number": {
                                "equals": phone_number
                            }
                        },
                        {
                            "property": "CreatedAt",
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
        debug!("Found {} spins today for phone number: {}", count, phone_number);
        Ok(count >= self.daily_spin_limit as usize)
    }
}

#[async_trait]
impl NotionRepository for NotionClient {
    async fn create_entry(&self, spin_result: SpinResult) -> Result<(), Error> {
        info!(
            "Creating new spin result for {} with ticket {}",
            spin_result.name, spin_result.ticket
        );

        if self.has_reached_spin_limit(&spin_result.phone_number).await? {
            warn!(
                "Daily spin limit reached for phone number: {}",
                spin_result.phone_number
            );
            return Err(Error::SpinLimitReached);
        }

        let properties = self.build_properties(&spin_result);
        
        let response = self.client
            .post("https://api.notion.com/v1/pages")
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Notion-Version", "2022-06-28")
            .json(&json!({
                "parent": { "database_id": self.database_id },
                "properties": properties
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Notion API error: {}", error_text);
            return Err(Error::NotionApi(error_text));
        }

        info!("Successfully created spin result");
        Ok(())
    }

    async fn get_entries(&self) -> Result<Vec<SpinResult>, Error> {
        debug!("Fetching all spin results");
        
        let response = self.client
            .post(format!("https://api.notion.com/v1/databases/{}/query", self.database_id))
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
            
            let name = properties["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();
            
            let phone_number = properties["Phone"]["phone_number"]
                .as_str()
                .unwrap_or("")
                .to_string();
            
            let ticket = properties["Ticket"]["number"]
                .as_i64()
                .unwrap_or(0) as i32;
            
            let reward = properties["Reward"]["rich_text"][0]["text"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            spin_results.push(SpinResult {
                name,
                phone_number,
                ticket,
                reward,
            });
        }

        info!("Successfully fetched {} spin results", spin_results.len());
        Ok(spin_results)
    }

    async fn update_entry(&self, page_id: &str, spin_result: SpinResult) -> Result<(), Error> {
        info!(
            "Updating spin result {} for {} with ticket {}",
            page_id, spin_result.name, spin_result.ticket
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
            error!("Failed to update spin result {}: {}", page_id, error_text);
            return Err(Error::NotionApi(error_text));
        }

        info!("Successfully updated spin result {}", page_id);
        Ok(())
    }

    async fn delete_entry(&self, page_id: &str) -> Result<(), Error> {
        info!("Deleting spin result {}", page_id);
        
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
            error!("Failed to delete spin result {}: {}", page_id, error_text);
            return Err(Error::NotionApi(error_text));
        }

        info!("Successfully deleted spin result {}", page_id);
        Ok(())
    }
} 