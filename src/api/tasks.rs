use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub status: String,
    pub progress: u32,
    pub total: u32,
}

pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<String, Task>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn create_task(&self, total: u32) -> String {
        let id = Uuid::new_v4().to_string();
        let task = Task {
            id: id.clone(),
            status: "pending".to_string(),
            progress: 0,
            total,
        };
        self.tasks.write().await.insert(id.clone(), task);
        id
    }
    
    pub async fn get_task(&self, id: &str) -> Option<Task> {
        self.tasks.read().await.get(id).cloned()
    }
    
    pub async fn update_progress(&self, id: &str, progress: u32) {
        if let Some(task) = self.tasks.write().await.get_mut(id) {
            task.progress = progress;
            task.status = if progress >= task.total {
                "completed".to_string()
            } else {
                "running".to_string()
            };
        }
    }
}
