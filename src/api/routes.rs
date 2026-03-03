use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use uuid::Uuid;

use super::models::*;

pub fn create_router() -> Router {
    Router::new()
        .route("/drama/create", post(create_task))
        .route("/drama/:id", get(get_task))
}

async fn create_task(
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let task_id = Uuid::new_v4().to_string();
    
    Ok(Json(TaskResponse {
        task_id,
        status: "pending".to_string(),
    }))
}

async fn get_task() -> Result<Json<TaskResponse>, StatusCode> {
    Ok(Json(TaskResponse {
        task_id: "test".to_string(),
        status: "completed".to_string(),
    }))
}
