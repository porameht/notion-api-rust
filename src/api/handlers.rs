use axum::{
    extract::{State, Path},
    response::Json,
    http::StatusCode,
};
use crate::{
    domain::models::SpinResult,
    application::services::NotionService,
    infrastructure::notion::NotionClient,
};

pub async fn create_spin_result(
    State(service): State<NotionService<NotionClient>>,
    Json(spin_result): Json<SpinResult>,
) -> Result<StatusCode, StatusCode> {
    service
        .create_spin_result(spin_result)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
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

// Implement other handlers... 