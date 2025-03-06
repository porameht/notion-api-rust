use dotenv::dotenv;
use reqwest::{Client, header};
use serde_json::json;
use std::env;
use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "Spin Results Database")]
    name: String,
    
    #[arg(short, long)]
    page_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    let args = Args::parse();
    
    println!("Starting Notion database creation...");
    println!("Database name: {}", args.name);
    
    let api_token = env::var("NOTION_API_TOKEN")
        .expect("NOTION_API_TOKEN must be set");
    
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&format!("Bearer {}", api_token))?,
    );
    headers.insert(
        "Notion-Version",
        header::HeaderValue::from_static("2022-06-28"),
    );
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    
    let page_id = if let Some(id) = args.page_id {
        println!("Using provided page ID: {}", id);
        format_page_id(&id)
    } else {
        env::var("NOTION_PAGE_ID")
            .expect("NOTION_PAGE_ID must be set")
    };

    println!("Make sure page ID {} is shared with your integration", page_id);
    println!("Visit https://www.notion.so/my-integrations to manage your integration");
    
    let response = client
        .post("https://api.notion.com/v1/databases")
        .headers(headers)
        .json(&json!({
            "parent": {
                "type": "page_id",
                "page_id": page_id
            },
            "title": [
                {
                    "type": "text",
                    "text": {
                        "content": format!("Spin Results Database - {}", args.name)
                    }
                }
            ],
            "properties": {
                "key": {
                    "title": {}
                },
                "datetime": {
                    "date": {}
                },
                "number": {
                    "number": {
                        "format": "number"
                    }
                },
                "is_win": {
                    "checkbox": {}
                },
                "checked": {
                    "checkbox": {}
                }
            }
        }))
        .send()
        .await?;
    
    if response.status().is_success() {
        let database_info: serde_json::Value = response.json().await?;
        println!("✅ Database created successfully!");
        println!("🚀 Database ID: {}", database_info["id"].as_str().unwrap_or("Unknown"));
        println!("Make sure to update your .env file with this new database ID as NOTION_PAGE_ID");
    } else {
        let error_text = response.text().await?;
        eprintln!("❌ Failed to create database: {}", error_text);
    }
    
    Ok(())
}

fn format_page_id(id: &str) -> String {
    if id.contains('-') {
        id.to_string()
    } else {
        format!(
            "{}-{}-{}-{}-{}", 
            &id[0..8], 
            &id[8..12], 
            &id[12..16], 
            &id[16..20], 
            &id[20..]
        )
    }
} 