use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::{models::*, tasks::TaskManager};

pub fn create_router(task_mgr: Arc<TaskManager>) -> Router {
    Router::new()
        .route("/drama/create", post(create_task))
        .route("/drama/:id", get(get_task))
        .with_state(task_mgr)
}

async fn create_task(
    State(mgr): State<Arc<TaskManager>>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let task_id = mgr.create_task(req.episodes).await;
    
    Ok(Json(TaskResponse {
        task_id,
        status: "pending".to_string(),
    }))
}

async fn get_task(
    State(mgr): State<Arc<TaskManager>>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let task = mgr.get_task(&id).await
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(TaskResponse {
        task_id: task.id,
        status: task.status,
    }))
}
