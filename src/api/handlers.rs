use axum::{
    extract::{State, Path},
    response::Json,
    http::StatusCode,
};
use crate::{
    domain::models::{SpinResult, SpinRequest, SpinResponse, WheelRequest, WheelResponse, GameType},
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
    match service.create_spin_result(spin_result, GameType::Spin).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(Error::SpinLimitReached) => Err(StatusCode::TOO_MANY_REQUESTS),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_spin_results(
    State(service): State<NotionService<NotionClient>>,
) -> Result<Json<Vec<SpinResult>>, StatusCode> {
    service
        .get_spin_results(GameType::Spin)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_spin_result(
    State(service): State<NotionService<NotionClient>>,
    Path(page_id): Path<String>,
    Json(spin_result): Json<SpinResult>,
) -> StatusCode {
    match service.update_spin_result(&page_id, spin_result, GameType::Spin).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_spin_result(
    State(service): State<NotionService<NotionClient>>,
    Path(page_id): Path<String>,
) -> StatusCode {
    match service.delete_spin_result(&page_id, GameType::Spin).await {
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
    let win_chance = 0.00; // 0% chance to win
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
            game_type: Some("Spin".to_string()),
        };
        
        if let Err(err) = service.create_spin_result(spin_result, GameType::Spin).await {
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

pub async fn wheel_result(
    State(service): State<NotionService<NotionClient>>,
    Json(request): Json<WheelRequest>,
) -> Result<Json<WheelResponse>, StatusCode> {
    // Initialize the RNG and handle any potential errors
    let mut rng = match SmallRng::from_entropy() {
        rng => rng,
    };
    
    // Define the prize weights for the wheel (copied from frontend)
    let weights = [
        0,   // รับเครดิต 500 - not possible to land
        30,  // หมุนฟรี 1 ครั้ง - medium chance
        5,   // รับเครดิต 50 - low chance
        35,  // แย่จัง - high chance
        0,   // รับเครดิต 300 - not possible to land
        30,  // หมุนฟรี 1 ครั้ง - medium chance
        5,   // รับเครดิต 100 - low chance
        35   // แย่จัง - high chance
    ];

    // Define the prize names (copied from frontend)
    let slice_prizes = [
        "รับเครดิต 500", 
        "หมุนฟรี 1 ครั้ง", 
        "รับเครดิต 50", 
        "แย่จัง", 
        "รับเครดิต 300", 
        "หมุนฟรี 1 ครั้ง", 
        "รับเครดิต 100", 
        "แย่จัง"
    ];
    
    // Calculate the total weight
    let total_weight: u32 = weights.iter().sum();
    if total_weight == 0 {
        // Prevent division by zero or other issues with zero weight
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // Generate a random number between 0 and the total weight
    let random_weight = rng.gen_range(0..total_weight);
    
    // Find the prize based on the random weight
    let mut cumulative_weight = 0;
    let mut prize_index = 0;
    
    for (i, &weight) in weights.iter().enumerate() {
        cumulative_weight += weight;
        if random_weight < cumulative_weight {
            prize_index = i;
            break;
        }
    }
    
    // Safety check to ensure prize_index is valid
    if prize_index >= slice_prizes.len() {
        prize_index = slice_prizes.len() - 1;
    }
    
    // Get the prize name
    let prize_name = slice_prizes[prize_index].to_string();
    
    // Determine if it's a win by checking specific prize indices
    // Only prize_index 2 (รับเครดิต 50) and prize_index 6 (รับเครดิต 100) are winning results
    let is_win = prize_index == 2 || prize_index == 6;
    
    // Create the response first, so we can return it even if saving to Notion fails
    let response = WheelResponse {
        prize_index,
        prize_name,
        is_win,
    };
    
    // Try to save to Notion if it's a win, but don't fail the whole request if this fails
    if is_win {
        let key = request.key.unwrap_or_else(|| Utc::now().timestamp_millis().to_string());
        let now = match Utc::now().to_rfc3339() {
            datetime => datetime,
        };
        
        // For storing in database, we'll convert the prize_index to a number
        let number = prize_index as i32;
        
        let spin_result = SpinResult {
            key,
            datetime: now,
            number,
            is_win,
            checked: false,
            game_type: Some("Wheel".to_string()),
        };
        
        // Fire-and-forget approach: try to save but return the response regardless
        // This prevents the 500 error from propagating to the client
        let _ = service.create_spin_result(spin_result, GameType::Wheel).await;
    }
    
    // Always return the response, even if saving to Notion failed
    Ok(Json(response))
}

// Implement other handlers... 