use axum::{
    extract::{State, Path},
    response::Json,
    http::StatusCode,
};
use crate::{
    domain::models::{SpinResult, SpinRequest, SpinResponse},
    domain::repository::Error,
    application::services::NotionService,
    infrastructure::notion::NotionClient,
};
use rand::{rngs::SmallRng, SeedableRng, Rng};
use chrono::Utc;

pub async fn create_spin_result(
    State(service): State<NotionService<NotionClient>>,
    Json(spin_result): Json<SpinResult>,
) -> Result<StatusCode, StatusCode> {
    match service.create_spin_result(spin_result).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(Error::SpinLimitReached) => Err(StatusCode::TOO_MANY_REQUESTS),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_spin_results(
    State(service): State<NotionService<NotionClient>>,
) -> Result<Json<Vec<SpinResult>>, StatusCode> {
    service
        .get_spin_results()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_spin_result(
    State(service): State<NotionService<NotionClient>>,
    Path(page_id): Path<String>,
    Json(spin_result): Json<SpinResult>,
) -> StatusCode {
    match service.update_spin_result(&page_id, spin_result).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_spin_result(
    State(service): State<NotionService<NotionClient>>,
    Path(page_id): Path<String>,
) -> StatusCode {
    match service.delete_spin_result(&page_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_root() -> &'static str {
    "Notion API is running"
}

pub async fn spin_result(
    State(service): State<NotionService<NotionClient>>,
    Json(request): Json<SpinRequest>,
) -> Result<Json<SpinResponse>, StatusCode> {
    // Generate three random numbers using a thread-safe RNG
    let mut rng = SmallRng::from_entropy();
    let win_chance = 0.01; // 1% chance to win
    let lucky_string = "5".to_string();
    
    // Determine if this is a winning spin
    let (numbers, is_win) = if rng.gen::<f64>() < win_chance {
        // Create winning combination (three 5s)
        (vec![lucky_string.clone(), lucky_string.clone(), lucky_string.clone()], true)
    } else {
        // Generate non-winning numbers
        let mut non_winning_numbers = Vec::with_capacity(3);
        
        for _ in 0..3 {
            // Generate a number between 0-9
            let num = rng.gen_range(0..10).to_string();
            non_winning_numbers.push(num);
        }
        
        // Make sure it's not a winning combination
        if non_winning_numbers.iter().all(|n| n == &lucky_string) {
            non_winning_numbers[0] = if lucky_string == "0" { 
                "1".to_string() 
            } else { 
                "0".to_string() 
            };
        }
        
        (non_winning_numbers, false)
    };
    
    // Save to Notion only if it's a win
    if is_win {
        let key = request.key.unwrap_or_else(|| Utc::now().timestamp_millis().to_string());
        let now = Utc::now().to_rfc3339();
        
        // Join the numbers to a single integer
        let number_str = numbers.join("");
        let number = number_str.parse::<i32>().unwrap_or(0);
        
        let spin_result = SpinResult {
            key,
            datetime: now,
            number,
            is_win,
            checked: false,
        };
        
        if let Err(err) = service.create_spin_result(spin_result).await {
            match err {
                Error::SpinLimitReached => return Err(StatusCode::TOO_MANY_REQUESTS),
                _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
    
    // Return response with the numbers and win status
    let response = SpinResponse {
        numbers,
        is_win,
    };
    
    Ok(Json(response))
}

// Implement other handlers... 