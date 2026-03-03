use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub project_name: String,
    pub genre: String,
    pub episodes: u32,
    pub script_provider: String,
    pub voice_provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub task_id: String,
    pub episode: u32,
    pub total: u32,
    pub status: String,
    pub message: String,
}
