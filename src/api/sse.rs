use axum::{
    response::sse::{Event, Sse},
    extract::{Path, State},
};
use futures::stream::{self, Stream};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::time::sleep;

use super::tasks::TaskManager;

pub async fn progress_stream(
    State(mgr): State<Arc<TaskManager>>,
    Path(task_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::unfold(0, move |progress| {
        let task_id = task_id.clone();
        let mgr = mgr.clone();
        
        async move {
            if progress >= 100 {
                return None;
            }
            
            sleep(Duration::from_secs(1)).await;
            
            let event = Event::default()
                .json_data(serde_json::json!({
                    "taskId": task_id,
                    "progress": progress,
                    "status": "running"
                }))
                .unwrap();
            
            Some((Ok(event), progress + 10))
        }
    });
    
    Sse::new(stream)
}
