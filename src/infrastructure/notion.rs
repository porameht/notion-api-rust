use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::domain::{
    models::*,
    repository::{NotionRepository, Error},
};

#[derive(Clone)]
pub struct NotionClient {
    client: Client,
    database_id: String,
    api_token: String,
}

impl NotionClient {
    pub fn new(database_id: String, api_token: String) -> Self {
        let client = Client::new();
        Self {
            client,
            database_id,
            api_token,
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
}

#[async_trait]
impl NotionRepository for NotionClient {
    async fn create_entry(&self, spin_result: SpinResult) -> Result<(), Error> {
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
            return Err(Error::NotionApi(response.text().await?));
        }

        Ok(())
    }

    async fn get_entries(&self) -> Result<Vec<SpinResult>, Error> {
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

        Ok(spin_results)
    }

    async fn update_entry(&self, page_id: &str, spin_result: SpinResult) -> Result<(), Error> {
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
            return Err(Error::NotionApi(response.text().await?));
        }

        Ok(())
    }

    async fn delete_entry(&self, page_id: &str) -> Result<(), Error> {
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
            return Err(Error::NotionApi(response.text().await?));
        }

        Ok(())
    }
} 