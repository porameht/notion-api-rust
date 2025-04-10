use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinResult {
    pub key: String,
    pub datetime: String,
    pub number: i32,
    pub is_win: bool,
    pub checked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinRequest {
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinResponse {
    pub numbers: Vec<String>,
    pub is_win: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionProperties {
    pub key: NotionTitle,
    pub datetime: NotionDate,
    pub number: NotionNumber,
    pub is_win: NotionCheckbox,
    pub checked: NotionCheckbox,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionTitle {
    pub r#type: String,
    pub title: Vec<NotionText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionDate {
    pub r#type: String,
    pub date: NotionDateContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionDateContent {
    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionNumber {
    pub r#type: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionCheckbox {
    pub r#type: String,
    pub checkbox: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionPhoneNumber {
    pub r#type: String,
    pub phone_number: String,
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