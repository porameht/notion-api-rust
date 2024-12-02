use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinResult {
    pub name: String,
    pub phone_number: String,
    pub ticket: i32,
    pub reward: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionProperties {
    pub Name: NotionTitle,
    pub Phone: NotionPhoneNumber,
    pub Ticket: NotionNumber,
    pub Reward: NotionRichText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionTitle {
    pub r#type: String,
    pub title: Vec<NotionText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionPhoneNumber {
    pub r#type: String,
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionNumber {
    pub r#type: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionRichText {
    pub r#type: String,
    pub rich_text: Vec<NotionText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionText {
    pub r#type: String,
    pub text: NotionTextContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionTextContent {
    pub content: String,
} 