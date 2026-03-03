use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// 批量并发执行任务
pub struct BatchExecutor {
    max_concurrent: usize,
}

impl BatchExecutor {
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent }
    }
    
    /// 并发执行多个任务,限制并发数
    pub async fn execute<F, Fut, T>(&self, tasks: Vec<F>) -> Vec<Result<T>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let mut handles = vec![];
        
        for task in tasks {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let handle = tokio::spawn(async move {
                let result = task().await;
                drop(permit);
                result
            });
            handles.push(handle);
        }
        
        let mut results = vec![];
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        results
    }
}
